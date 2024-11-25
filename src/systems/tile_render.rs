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
        let get_resource = world.get_resource::<TileMap>();
        let tilemap = get_resource.unwrap();

        let frame = pixels.frame_mut();

        // Calculate the camera's top-left position by subtracting half screen dimensions
        let camera_left = camera.position().x - (SCREEN_WIDTH as f32 / 2.0);
        let camera_top = camera.position().y - (SCREEN_HEIGHT as f32 / 2.0);

        // Calculate visible tile range based on camera's top-left position
        let start_tile_x = (camera_left / tilemap.tilesize as f32).floor() as i64 - 1;
        let start_tile_y = (camera_top / tilemap.tilesize as f32).floor() as i64 - 1;
        let columns = (SCREEN_WIDTH / tilemap.tilesize) as i64 + 2;
        let rows = (SCREEN_HEIGHT / tilemap.tilesize) as i64 + 2;

        // Iterate through visible tiles
        for y in start_tile_y..(start_tile_y + rows) {
            for x in start_tile_x..(start_tile_x + columns) {
                let Some(tile) = tilemap.tiles.get(&(x, y)) else {
                    continue;
                };
                // Convert world position to screen position using camera
                // TODO : use camera.world_to_screen ??
                let screen_x =
                    (tile.world_x as f32 - camera.position().x) + (SCREEN_WIDTH as f32 / 2.0);
                let screen_y =
                    (tile.world_y as f32 - camera.position().y) + (SCREEN_HEIGHT as f32 / 2.0);

                draw_tile(
                    frame,
                    &tilemap.tileset_pixels,
                    tilemap.tileset_width,
                    tile,
                    tilemap.tilesize,
                    screen_x as i32,
                    screen_y as i32,
                );

                if !tile.traversable {
                    draw_debug_overlay(frame, screen_x as i32, screen_y as i32, tilemap.tilesize);
                }
            }
        }
    }
}

fn draw_tile(
    frame: &mut [u8],
    tileset: &[u8],
    tileset_width: u32,
    tile: &TileData,
    tilesize: u32,
    screen_x: i32,
    screen_y: i32,
) {
    // Calculate source coordinates in tileset
    let src_x = (tile.tile_id % (tileset_width / tilesize) as i32) * tilesize as i32;
    let src_y = (tile.tile_id / (tileset_width / tilesize) as i32) * tilesize as i32;

    for y in 0..tilesize as i32 {
        for x in 0..tilesize as i32 {
            let screen_pixel_x = screen_x + x;
            let screen_pixel_y = screen_y + y;

            // Skip if pixel would be off screen
            if screen_pixel_x < 0
                || screen_pixel_y < 0
                || screen_pixel_x >= SCREEN_WIDTH as i32
                || screen_pixel_y >= SCREEN_HEIGHT as i32
            {
                continue;
            }

            let src_pixel_x = src_x + x;
            let src_pixel_y = src_y + y;

            let src_idx = ((src_pixel_y * tileset_width as i32 + src_pixel_x) * 4) as usize;
            let dst_idx = ((screen_pixel_y * SCREEN_WIDTH as i32 + screen_pixel_x) * 4) as usize;

            frame[dst_idx..dst_idx + 4].copy_from_slice(&tileset[src_idx..src_idx + 4]);
        }
    }
}

/// Draws a semi-transparent red overlay on non-traversable tiles
fn draw_debug_overlay(frame: &mut [u8], screen_x: i32, screen_y: i32, tilesize: u32) {
    for y in 0..tilesize as i32 {
        for x in 0..tilesize as i32 {
            let screen_pixel_x = screen_x + x;
            let screen_pixel_y = screen_y + y;

            // Skip if pixel would be off screen
            if screen_pixel_x < 0
                || screen_pixel_y < 0
                || screen_pixel_x >= SCREEN_WIDTH as i32
                || screen_pixel_y >= SCREEN_HEIGHT as i32
            {
                continue;
            }

            let dst_idx = ((screen_pixel_y * SCREEN_WIDTH as i32 + screen_pixel_x) * 4) as usize;

            // Blend red overlay with existing pixel
            // Increase red channel and slightly decrease other channels
            frame[dst_idx] = (frame[dst_idx] as u16 * 3 / 4 + 64) as u8; // Red
            frame[dst_idx + 1] = (frame[dst_idx + 1] as f32 * 0.7) as u8; // Green
            frame[dst_idx + 2] = (frame[dst_idx + 2] as f32 * 0.7) as u8; // Blue
                                                                          // Alpha remains unchanged
        }
    }
}
