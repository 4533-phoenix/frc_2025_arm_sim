use bevy::prelude::*;
use std::fs::File;
use std::io::Write;

const ARM_LENGTH: f32 = 39.37;
const GRID_RESOLUTION: f32 = 2.0; // Step size for grid

#[derive(Component)]
pub struct IntakeMarker;

#[derive(Component)]
pub struct GridMarker;

#[derive(Resource)]
pub struct GridState {
    pub current_x: f32,
    pub current_y: f32,
    pub step_size: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub completed: bool,
    pub collision_grid: Vec<Vec<bool>>,
}

impl GridState {
    pub fn new() -> Self {
        let min_x = -ARM_LENGTH;
        let max_x = ARM_LENGTH;
        let min_y = -13.65 - ARM_LENGTH;
        let max_y = 31.75 + ARM_LENGTH;
        let step_size = GRID_RESOLUTION;

        // Calculate grid dimensions
        let width = ((max_x - min_x) / step_size).ceil() as usize + 1;
        let height = ((max_y - min_y) / step_size).ceil() as usize + 1;

        Self {
            current_x: min_x,
            current_y: min_y,
            step_size,
            min_x,
            max_x,
            min_y,
            max_y,
            completed: false,
            collision_grid: vec![vec![false; width]; height],
        }
    }

    pub fn get_grid_indices(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let x_idx = ((x - self.min_x) / self.step_size).floor() as usize;
        let y_idx = ((y - self.min_y) / self.step_size).floor() as usize;

        if x_idx < self.collision_grid[0].len() && y_idx < self.collision_grid.len() {
            Some((x_idx, y_idx))
        } else {
            None
        }
    }

    pub fn mark_collision(&mut self, x: f32, y: f32) {
        if let Some((x_idx, y_idx)) = self.get_grid_indices(x, y) {
            self.collision_grid[y_idx][x_idx] = true;
        }
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let mut file = File::create("collision_grid.bin")?;

        // Write grid dimensions
        let width = self.collision_grid[0].len() as u32;
        let height = self.collision_grid.len() as u32;
        file.write_all(&width.to_le_bytes())?;
        file.write_all(&height.to_le_bytes())?;

        // Write grid boundaries
        file.write_all(&self.min_x.to_le_bytes())?;
        file.write_all(&self.max_x.to_le_bytes())?;
        file.write_all(&self.min_y.to_le_bytes())?;
        file.write_all(&self.max_y.to_le_bytes())?;
        file.write_all(&self.step_size.to_le_bytes())?;

        // Write collision data
        for row in &self.collision_grid {
            for &cell in row {
                file.write_all(&[cell as u8])?;
            }
        }

        Ok(())
    }
}
