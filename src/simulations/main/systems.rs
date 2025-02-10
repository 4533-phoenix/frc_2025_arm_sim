use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::simulations::main::components::*;

pub fn update_mouse_position(
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

pub fn handle_control_mode(keys: Res<ButtonInput<KeyCode>>, mut control_mode: ResMut<ControlMode>) {
    if keys.just_pressed(KeyCode::Space) {
        *control_mode = match *control_mode {
            ControlMode::CodeControl => ControlMode::CursorFollow,
            ControlMode::CursorFollow => ControlMode::CodeControl,
        };
        println!("Switched control mode");
    }
}

pub fn apply_intake_force(
    control_mode: Res<ControlMode>,
    mouse_pos: Res<MouseWorldPos>,
    mut intake_query: Query<(&Transform, &mut ExternalForce), With<IntakeMarker>>,
) {
    if matches!(*control_mode, ControlMode::CursorFollow) {
        for (transform, mut ext_force) in intake_query.iter_mut() {
            let direction = (mouse_pos.0 - transform.translation.truncate()).normalize();
            ext_force.force = direction * 500000.0;
        }
    } else {
        // Reset forces when not in cursor follow mode
        for (_, mut ext_force) in intake_query.iter_mut() {
            ext_force.force = Vec2::ZERO;
        }
    }
}
