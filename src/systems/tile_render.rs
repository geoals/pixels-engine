use std::time::Duration;

use crate::{
    camera::Camera,
    input::Input,
    tile::{TileData, TileMap},
    World, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::System;

pub struct TileRenderSystem;

impl System for TileRenderSystem {
    fn update(
        &self,
        world: &World,
        pixels: &mut pixels::Pixels,
        _input: &Input,
        _delta_time: Duration,
    ) {
        let camera = world.get_resource::<Camera>().unwrap();
        let tilemap = world.get_resource::<TileMap>().unwrap();
        let current_level = tilemap.current_level();

        let frame = pixels.frame_mut();

        let camera_left = camera.position().x - (SCREEN_WIDTH as f32 / 2.0);
        let camera_top = camera.position().y - (SCREEN_HEIGHT as f32 / 2.0);

        // Calculate visible tile range based on camera's top-left position
        let start_tile_x = (camera_left / tilemap.tilesize as f32).floor() as i64;
        let start_tile_y = (camera_top / tilemap.tilesize as f32).floor() as i64;
        let columns = (SCREEN_WIDTH / tilemap.tilesize as u32) as i64 + 1;
        let rows = (SCREEN_HEIGHT / tilemap.tilesize as u32) as i64 + 1;

        // Iterate through visible tiles
        for y in start_tile_y..(start_tile_y + rows) {
            for x in start_tile_x..(start_tile_x + columns) {
                let Some(tile) = current_level.tiles.get(&(x, y)) else {
                    continue;
                };

                let screen_pos = camera.world_to_screen(tile.position.into());

                // Round to prevent subpixel positioning
                // This is required to prevent off by one pixel jitter when moving up or left
                let screen_x = screen_pos.x.round() as i64;
                let screen_y = screen_pos.y.round() as i64;

                draw_tile(
                    frame,
                    &current_level.tileset_pixels,
                    current_level.tileset_width,
                    tile,
                    tilemap.tilesize,
                    screen_x,
                    screen_y,
                );
            }
        }
    }
}

fn draw_tile(
    frame: &mut [u8],
    tileset: &[u8],
    tileset_width: u32,
    tile: &TileData,
    tilesize: i64,
    screen_x: i64,
    screen_y: i64,
) {
    for y in 0..tilesize {
        for x in 0..tilesize {
            let screen_pixel_x = screen_x + x;
            let screen_pixel_y = screen_y + y;

            // Skip if pixel would be off screen
            if screen_pixel_x < 0
                || screen_pixel_y < 0
                || screen_pixel_x >= SCREEN_WIDTH as i64
                || screen_pixel_y >= SCREEN_HEIGHT as i64
            {
                continue;
            }

            let src_pixel_x = tile.tileset_position.x + x;
            let src_pixel_y = tile.tileset_position.y + y;

            let src_idx = ((src_pixel_y * tileset_width as i64 + src_pixel_x) * 4) as usize;
            let dst_idx = ((screen_pixel_y * SCREEN_WIDTH as i64 + screen_pixel_x) * 4) as usize;

            frame[dst_idx..dst_idx + 4].copy_from_slice(&tileset[src_idx..src_idx + 4]);
        }
    }
}
