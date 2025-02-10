use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const ELEVATOR: Group = Group::GROUP_1;
pub const INTAKE: Group = Group::GROUP_2;

#[derive(Component)]
pub struct IntakeMarker;

#[derive(Resource, Default)]
pub struct MouseWorldPos(pub Vec2);

#[derive(Resource, Default)]
pub enum ControlMode {
    #[default]
    CodeControl,
    CursorFollow,
}

#[derive(Resource)]
pub struct MotorJoints {
    pub elevator: Entity,
    pub arm: Entity,
    pub elevator_body: Entity, // Added
    pub arm_body: Entity,      // Added
}
