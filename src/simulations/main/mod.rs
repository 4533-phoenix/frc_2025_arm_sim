mod code_control;
mod components;
mod physics;
mod systems;
mod kinematics;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use code_control::*;
use components::*;
use systems::*;

pub fn run() -> App {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        RapierDebugRenderPlugin::default(),
    ))
    .init_resource::<MouseWorldPos>()
    .init_resource::<ControlMode>()
    .init_resource::<TargetPosition>()
    .add_systems(Startup, (setup_graphics, physics::setup_physics))
    .add_systems(
        Update,
        (
            update_mouse_position,
            apply_intake_force,
            handle_control_mode,
            update_code_motors,
        ),
    );
    app
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
