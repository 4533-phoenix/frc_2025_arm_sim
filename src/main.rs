use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const ELEVATOR: Group = Group::GROUP_1;
const INTAKE: Group = Group::GROUP_2;

#[derive(Component)]
struct IntakeMarker;

#[derive(Resource, Default)]
struct MouseWorldPos(Vec2);

#[derive(Resource, Default)]
enum ControlMode {
    #[default]
    CodeControl,
    CursorFollow,
}

#[derive(Resource)]
struct MotorJoints {
    elevator: Entity,
    arm: Entity,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .init_resource::<MouseWorldPos>()
        .init_resource::<ControlMode>()
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, (
            update_mouse_position,
            apply_intake_force,
            handle_control_mode,
            update_motors,
        ))
        .run();
}

fn update_mouse_position(
    mut mouse_pos: ResMut<MouseWorldPos>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    let window = windows.single();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        mouse_pos.0 = world_pos;
    }
}

fn handle_control_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut control_mode: ResMut<ControlMode>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *control_mode = match *control_mode {
            ControlMode::CodeControl => ControlMode::CursorFollow,
            ControlMode::CursorFollow => ControlMode::CodeControl,
        };
        println!("Switched control mode");
    }
}

fn update_motors(
    control_mode: Res<ControlMode>,
    mut joints: Query<&mut ImpulseJoint>,
    motor_joints: Res<MotorJoints>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if matches!(*control_mode, ControlMode::CodeControl) {
        // Control elevator motor
        if let Ok(mut joint) = joints.get_mut(motor_joints.elevator) {
            let motor_speed = if keys.pressed(KeyCode::KeyW) {
                1000.0
            } else if keys.pressed(KeyCode::KeyS) {
                -1000.0
            } else {
                0.0
            };
            joint.data.as_mut().set_motor_velocity(JointAxis::LinY, motor_speed, 1.0);
        }

        // Control arm motor
        if let Ok(mut joint) = joints.get_mut(motor_joints.arm) {
            let motor_speed = if keys.pressed(KeyCode::KeyD) {
                5.0
            } else if keys.pressed(KeyCode::KeyA) {
                -5.0
            } else {
                0.0
            };
            joint.data.as_mut().set_motor_velocity(JointAxis::AngX, motor_speed, 1.0);
        }
    }
}

fn apply_intake_force(
    control_mode: Res<ControlMode>,
    mouse_pos: Res<MouseWorldPos>,
    mut intake_query: Query<(&Transform, &mut ExternalForce), With<IntakeMarker>>,
) {
    if matches!(*control_mode, ControlMode::CursorFollow) {
        for (transform, mut ext_force) in intake_query.iter_mut() {
            let direction = (mouse_pos.0 - transform.translation.truncate()).normalize();
            ext_force.force = direction * 50000.0;
        }
    } else {
        // Reset forces when not in cursor follow mode
        for (_, mut ext_force) in intake_query.iter_mut() {
            ext_force.force = Vec2::ZERO;
        }
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        },
    ));
}

fn setup_physics(mut commands: Commands) {
    let elevator_body = commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(2.52, 48.25),
            Transform::from_xyz(0.0, 0.0, 0.0),
            CollisionGroups::new(ELEVATOR, INTAKE),
        ))
        .id();

    let carriage = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Collider::ball(5.08),
            Transform::from_xyz(5.08, -13.65, 0.0),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
        ))
        .id();

    let arm = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Collider::cuboid(19.685, 2.52),
            Transform::from_xyz(24.765, -13.4, 0.0),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
        ))
        .id();

    let intake_pivot = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            Collider::ball(5.08),
            Transform::from_xyz(44.45, -13.4, 0.0)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
        ))
        .id();

    let intake = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Collider::cuboid(19.05, 6.675),
            Transform::from_xyz(35.258, -3.808, 0.0)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            CollisionGroups::new(INTAKE, ELEVATOR),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
            IntakeMarker,
            ExternalForce::default(),
        ))
        .id();

    let joint_elevator_carriage = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(-5.08, 0.0))
        .limits([-13.65, 31.75])
        .motor_position(0.0, 1000.0, 100000.0); // Add motor to elevator

    let joint_carriage_arm = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(-19.685, 0.0))
        .motor_position(0.0, 5.0, 100000.0); // Add motor to arm joint

    let joint_arm_pivot = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(19.685, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    let joint_pivot_intake = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(0.0, -13.0));

    let elevator_joint = commands
        .spawn(ImpulseJoint::new(elevator_body, joint_elevator_carriage))
        .set_parent(carriage)
        .id();

    let arm_joint = commands
        .spawn(ImpulseJoint::new(carriage, joint_carriage_arm))
        .set_parent(arm)
        .id();

    // Store motor joint handles
    commands.insert_resource(MotorJoints {
        elevator: elevator_joint,
        arm: arm_joint,
    });

    commands
        .spawn(ImpulseJoint::new(arm, joint_arm_pivot))
        .set_parent(intake_pivot);

    commands
        .spawn(ImpulseJoint::new(intake_pivot, joint_pivot_intake))
        .set_parent(intake);
}
