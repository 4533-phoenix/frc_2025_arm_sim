use bevy::prelude::*;

const ARM_LENGTH: f32 = 39.37;
const INTAKE_OFFSET: f32 = 13.0;
const ELEVATOR_MIN: f32 = -13.65;
const ELEVATOR_OFFSET: f32 = -13.65;

pub struct ArmPosition {
    pub height: f32,      // Elevator height
    pub arm_angle: f32,   // Angle in radians
}

impl ArmPosition {
    pub fn from_target(target: Vec2) -> Option<Self> {
        // Adjust target position to account for intake offset
        let adjusted_x = target.x;
        let adjusted_y = target.y + INTAKE_OFFSET;

        // Calculate distance from elevator to target
        let distance = (adjusted_x.powi(2) + adjusted_y.powi(2)).sqrt();

        // Check if point is reachable
        if distance > ARM_LENGTH {
            return None;
        }

        // Calculate base angle using atan2
        let base_angle = adjusted_y.atan2(adjusted_x);

        // Calculate required elevator height
        let height = adjusted_y - ARM_LENGTH * base_angle.sin();
        
        // Validate elevator height is within bounds
        if height < ELEVATOR_MIN || height > 31.75 {
            return None;
        }

        Some(Self {
            height: height - ELEVATOR_OFFSET,  // Adjust for elevator offset
            arm_angle: base_angle,
        })
    }

    pub fn validate_with_grid(&self, grid: &Vec<Vec<bool>>, min_x: f32, min_y: f32, step: f32) -> bool {
        // Calculate arm endpoint position
        let arm_x = ARM_LENGTH * self.arm_angle.cos();
        let arm_y = (self.height + ELEVATOR_OFFSET) + ARM_LENGTH * self.arm_angle.sin();

        // Convert to grid coordinates
        let grid_x = ((arm_x - min_x) / step).floor() as usize;
        let grid_y = ((arm_y - min_y) / step).floor() as usize;

        // Check if position is within grid bounds and collision-free
        grid_y < grid.len() 
            && grid_x < grid[0].len() 
            && !grid[grid_y][grid_x]
    }
}
