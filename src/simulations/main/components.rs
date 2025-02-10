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

#[derive(Resource)]
pub struct CollisionGrid {
    pub grid: Vec<Vec<bool>>,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub step_size: f32,
}

impl CollisionGrid {
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(path)?;
        let mut read_bytes = |count: usize| -> std::io::Result<Vec<u8>> {
            let mut buffer = vec![0u8; count];
            file.read_exact(&mut buffer)?;
            Ok(buffer)
        };

        // Read dimensions
        let width = u32::from_le_bytes(read_bytes(4)?.try_into().unwrap());
        let height = u32::from_le_bytes(read_bytes(4)?.try_into().unwrap());

        // Read boundaries
        let min_x = f32::from_le_bytes(read_bytes(4)?.try_into().unwrap());
        let max_x = f32::from_le_bytes(read_bytes(4)?.try_into().unwrap());
        let min_y = f32::from_le_bytes(read_bytes(4)?.try_into().unwrap());
        let max_y = f32::from_le_bytes(read_bytes(4)?.try_into().unwrap());
        let step_size = f32::from_le_bytes(read_bytes(4)?.try_into().unwrap());

        // Read grid data
        let mut grid = vec![vec![false; width as usize]; height as usize];
        for y in 0..height as usize {
            for x in 0..width as usize {
                grid[y][x] = read_bytes(1)?[0] != 0;
            }
        }

        Ok(Self {
            grid,
            min_x,
            max_x,
            min_y,
            max_y,
            step_size,
        })
    }
}
