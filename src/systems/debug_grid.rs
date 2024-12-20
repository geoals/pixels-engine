use super::System;
use crate::input::Input;
use crate::resource::Resources;
use crate::vec2::Vec2;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use crate::TILE_SIZE;
use hecs::World;
use pixels::Pixels;
use std::time::Duration;

pub struct DebugGridSystem;

impl System for DebugGridSystem {
    fn update(
        &self,
        _world: &mut World,
        resources: &mut Resources,
        pixels: &mut Pixels,
        _input: &Input,
        _delta_time: Duration,
    ) {
        let camera = &resources.camera;
        let frame = pixels.frame_mut();

        // Calculate visible grid range
        let top_left = camera.screen_to_world(Vec2::ZERO);
        let bottom_right =
            camera.screen_to_world(Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));

        // Extend the range by one tile to ensure smooth scrolling
        let start_x = ((top_left.x / TILE_SIZE as f32).floor() - 1.0) as i32;
        let end_x = ((bottom_right.x / TILE_SIZE as f32).ceil() + 1.0) as i32;
        let start_y = ((top_left.y / TILE_SIZE as f32).floor() - 1.0) as i32;
        let end_y = ((bottom_right.y / TILE_SIZE as f32).ceil() + 1.0) as i32;

        // Draw horizontal lines
        for grid_y in start_y..end_y {
            let world_pos = Vec2::new(0.0, grid_y as f32 * TILE_SIZE as f32);
            let screen_pos = camera.world_to_screen(world_pos);
            let y = screen_pos.y as i32;

            if y >= 0 && y < SCREEN_HEIGHT as i32 {
                for x in 0..SCREEN_WIDTH {
                    let i = (4 * x + y as u32 * SCREEN_WIDTH * 4) as usize;
                    frame[i] = 255; // R
                    frame[i + 1] = 255; // G
                    frame[i + 2] = 255; // B
                    frame[i + 3] = 255; // A
                }
            }
        }

        // Draw vertical lines
        for grid_x in start_x..end_x {
            let world_pos = Vec2::new(grid_x as f32 * TILE_SIZE as f32, 0.0);
            let screen_pos = camera.world_to_screen(world_pos);
            let x = screen_pos.x as i32;

            if x >= 0 && x < SCREEN_WIDTH as i32 {
                for y in 0..SCREEN_HEIGHT {
                    let i = (4 * x as u32 + y * SCREEN_WIDTH * 4) as usize;
                    frame[i] = 255; // R
                    frame[i + 1] = 255; // G
                    frame[i + 2] = 255; // B
                    frame[i + 3] = 255; // A
                }
            }
        }
    }
}
