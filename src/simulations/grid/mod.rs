use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
use components::*;

const ELEVATOR: Group = Group::GROUP_1;
const INTAKE: Group = Group::GROUP_2;
const STEPS_PER_FRAME: usize = 10;

#[derive(Component)]
struct IntakeMaterial(Handle<ColorMaterial>);

#[derive(Resource)]
struct BatchResources {
    collision_mesh: Handle<Mesh>,
    safe_mesh: Handle<Mesh>,
    collision_material: Handle<ColorMaterial>,
    safe_material: Handle<ColorMaterial>,
}

pub fn run() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
    ))
    .insert_resource(GridState::new())
    .add_systems(
        Startup,
        (setup_graphics, setup_bodies, setup_batch_resources),
    )
    .add_systems(Update, check_grid_position);
    app
}

fn setup_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn elevator sensor
    commands.spawn((
        Collider::cuboid(2.52, 48.25),
        Sensor,
        Mesh2d(meshes.add(Rectangle::new(5.04, 96.5))),
        MeshMaterial2d(materials.add(Color::linear_rgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        CollisionGroups::new(ELEVATOR, INTAKE),
    ));

    // Store the default material
    let default_material = materials.add(Color::linear_rgb(0.2, 0.8, 0.2));

    // Spawn intake with marker component and material reference
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(38.1, 13.35))),
        MeshMaterial2d(default_material.clone()),
        Transform::from_xyz(-20.0, -20.0, 1.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
        IntakeMarker,
        IntakeMaterial(default_material),
    ));
}

fn setup_batch_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let batch_resources = BatchResources {
        collision_mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        safe_mesh: meshes.add(Rectangle::new(1.0, 1.0)),
        collision_material: materials.add(Color::linear_rgb(0.8, 0.2, 0.2)),
        safe_material: materials.add(Color::linear_rgb(0.2, 0.8, 0.2)),
    };
    commands.insert_resource(batch_resources);
}

fn check_grid_position(
    mut commands: Commands,
    mut grid_state: ResMut<GridState>,
    mut intake_query: Query<
        (
            &mut Transform,
            &mut MeshMaterial2d<ColorMaterial>,
            &IntakeMaterial,
        ),
        With<IntakeMarker>,
    >,
    physics_context: ReadDefaultRapierContext,
    batch_resources: Res<BatchResources>,
) {
    if grid_state.completed {
        return;
    }

    for _ in 0..STEPS_PER_FRAME {
        let current_x = grid_state.current_x;
        let current_y = grid_state.current_y;
        let max_x = grid_state.max_x;
        let max_y = grid_state.max_y;
        let min_x = grid_state.min_x;
        let step_size = grid_state.step_size;

        if let Ok((mut transform, mut material, default_material)) = intake_query.get_single_mut() {
            // Calculate intake position based on theoretical arm endpoint
            // Offset the intake by 13 units at 45 degrees from the arm endpoint
            let offset_distance = 13.0;
            let offset_angle = std::f32::consts::PI / 4.0; // 45 degrees
            
            // Position the intake relative to the theoretical arm endpoint
            let intake_x = current_x + offset_distance * offset_angle.cos();
            let intake_y = current_y + offset_distance * offset_angle.sin();
            
            transform.translation.x = intake_x;
            transform.translation.y = intake_y;

            // Rest of the collision checking remains the same
            let mut has_collision = false;
            physics_context.intersections_with_shape(
                transform.translation.truncate(),
                transform.rotation.xyz().z,
                &Collider::cuboid(19.05, 6.675),
                QueryFilter::new().groups(CollisionGroups::new(INTAKE, ELEVATOR)),
                |_entity| {
                    has_collision = true;
                    false
                },
            );

            // Update material based on collision state
            material.0 = if has_collision {
                batch_resources.collision_material.clone()
            } else {
                default_material.0.clone()
            };

            // Spawn batched marker
            commands.spawn((
                Mesh2d(if has_collision {
                    batch_resources.collision_mesh.clone()
                } else {
                    batch_resources.safe_mesh.clone()
                }),
                MeshMaterial2d(if has_collision {
                    batch_resources.collision_material.clone()
                } else {
                    batch_resources.safe_material.clone()
                }),
                Transform::from_xyz(current_x, current_y, -1.0),
                GridMarker,
            ));

            if has_collision {
                grid_state.mark_collision(current_x, current_y);
            }

            // Move to next position
            grid_state.current_x += step_size;
            if grid_state.current_x > max_x {
                grid_state.current_x = min_x;
                grid_state.current_y += step_size;

                if grid_state.current_y > max_y {
                    grid_state.completed = true;
                    println!(
                        "Grid check completed! Grid size: {}x{}",
                        grid_state.collision_grid[0].len(),
                        grid_state.collision_grid.len()
                    );

                    // Save grid to file
                    if let Err(e) = grid_state.save_to_file() {
                        println!("Failed to save collision grid: {}", e);
                    } else {
                        println!("Collision grid saved to collision_grid.bin");
                    }
                }
            }
        }

        if grid_state.completed {
            break;
        }
    }
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        },
    ));
}
