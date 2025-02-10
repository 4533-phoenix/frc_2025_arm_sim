use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::simulations::main::components::*;

const ELEVATOR_OFFSET: f32 = -13.65;

#[derive(Debug, Clone, Copy)]
pub enum PresetPosition {
    BottomLeft,
    BottomRight,
}

impl PresetPosition {
    pub fn height(&self) -> f32 {
        match self {
            Self::BottomLeft => 0.0,
            Self::BottomRight => 0.0,
        }
    }

    pub fn angle(&self) -> f32 {
        match self {
            Self::BottomLeft => 220.0_f32.to_radians(),
            Self::BottomRight => -40.0_f32.to_radians(),
        }
    }
}

#[derive(Resource)]
pub struct TargetPosition {
    pub current: PresetPosition,
}

impl Default for TargetPosition {
    fn default() -> Self {
        Self {
            current: PresetPosition::BottomRight,
        }
    }
}

pub fn update_code_motors(
    control_mode: Res<ControlMode>,
    mut joints: Query<&mut ImpulseJoint>,
    transforms: Query<&Transform>,
    motor_joints: Res<MotorJoints>,
    keys: Res<ButtonInput<KeyCode>>,
    mut target: ResMut<TargetPosition>,
) {
    // Print current positions
    if let Ok(transform) = transforms.get(motor_joints.elevator_body) {
        println!("Elevator position: {:.2}", transform.translation.y);
    }
    if let Ok(transform) = transforms.get(motor_joints.arm_body) {
        let angle_degrees = transform.rotation.to_euler(EulerRot::XYZ).2.to_degrees();
        println!("Arm angle: {:.2} degrees", angle_degrees);
    }

    if matches!(*control_mode, ControlMode::CodeControl) {
        // Handle position switching
        if keys.just_pressed(KeyCode::ArrowLeft) {
            target.current = PresetPosition::BottomLeft;
        } else if keys.just_pressed(KeyCode::ArrowRight) {
            target.current = PresetPosition::BottomRight;
        }

        // Update elevator position
        if let Ok(mut joint) = joints.get_mut(motor_joints.elevator) {
            joint.data.as_mut().set_motor_position(
                JointAxis::LinX,
                target.current.height() + ELEVATOR_OFFSET,
                5000.0,
                300.0,
            );
        }

        // Update arm position
        if let Ok(mut joint) = joints.get_mut(motor_joints.arm) {
            joint.data.as_mut().set_motor_position(
                JointAxis::AngX,
                target.current.angle(),
                5000.0,
                300.0,
            );
        }
    } else {
        // Disable elevator motor
        if let Ok(mut joint) = joints.get_mut(motor_joints.elevator) {
            joint
                .data
                .as_mut()
                .set_motor(JointAxis::LinX, 0.0, 0.0, 0.0, 0.0);
        }
        // Disable arm motor
        if let Ok(mut joint) = joints.get_mut(motor_joints.arm) {
            joint
                .data
                .as_mut()
                .set_motor(JointAxis::AngX, 0.0, 0.0, 0.0, 0.0);
        }
    }
}
