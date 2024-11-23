use std::time::Duration;

use crate::{
    camera::Camera,
    input::Input,
    tile::{TileData, TileMap, SPRITE_TILE_SIZE},
    World, HEIGHT, WIDTH,
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

        // Calculate visible tile range based on camera position
        let cam_pos = camera.position();
        let start_tile_x = (cam_pos.x / SPRITE_TILE_SIZE as f32) as i64;
        let start_tile_y = (cam_pos.y / SPRITE_TILE_SIZE as f32) as i64;
        let tiles_w = (WIDTH / SPRITE_TILE_SIZE) as i64 + 2;
        let tiles_h = (HEIGHT / SPRITE_TILE_SIZE) as i64 + 2;

        // Iterate through visible tiles
        for y in start_tile_y..(start_tile_y + tiles_h) {
            for x in start_tile_x..(start_tile_x + tiles_w) {
                if let Some(tile) = tilemap.tiles.get(&(x, y)) {
                    // Convert world position to screen position using camera
                    let screen_x = tile.world_x as f32 - cam_pos.x;
                    let screen_y = tile.world_y as f32 - cam_pos.y;

                    // Only draw if on screen
                    if screen_x >= -(SPRITE_TILE_SIZE as f32)
                        && screen_x < WIDTH as f32
                        && screen_y >= -(SPRITE_TILE_SIZE as f32)
                        && screen_y < HEIGHT as f32
                    {
                        draw_tile(
                            frame,
                            &tilemap.tileset_pixels,
                            tilemap.tileset_width,
                            tile,
                            screen_x as i32,
                            screen_y as i32,
                        );
                    }
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
    screen_x: i32,
    screen_y: i32,
) {
    let tile_size = 8;
    // Calculate source coordinates in tileset
    let src_x = (tile.tile_id % (tileset_width as i32 / tile_size)) * tile_size;
    let src_y = (tile.tile_id / (tileset_width as i32 / tile_size)) * tile_size;

    for y in 0..tile_size {
        for x in 0..tile_size {
            let screen_pixel_x = screen_x + x;
            let screen_pixel_y = screen_y + y;

            // Skip if pixel would be off screen
            if screen_pixel_x < 0
                || screen_pixel_y < 0
                || screen_pixel_x >= WIDTH as i32
                || screen_pixel_y >= HEIGHT as i32
            {
                continue;
            }

            let src_pixel_x = src_x + x;
            let src_pixel_y = src_y + y;

            let src_idx = ((src_pixel_y * tileset_width as i32 + src_pixel_x) * 4) as usize;
            let dst_idx = ((screen_pixel_y * WIDTH as i32 + screen_pixel_x) * 4) as usize;

            // Copy RGBA values
            frame[dst_idx..dst_idx + 4].copy_from_slice(&tileset[src_idx..src_idx + 4]);
        }
    }
}
