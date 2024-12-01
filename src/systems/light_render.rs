use std::time::Duration;

use pixels::Pixels;

use super::System;
use crate::{
    camera::Camera,
    components::{Light, Position},
    input::Input,
    resource::{LightMap, Resources},
    SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

pub struct LightRenderSystem;

impl System for LightRenderSystem {
    fn update(
        &self,
        _: &mut hecs::World,
        resources: &mut Resources,
        pixels: &mut Pixels,
        _: &Input,
        _: Duration,
    ) {
        let indoors = resources.tilemap.levels[&resources.current_level_id.0].indoors;
        if indoors {
            return;
        }

        render_lighting(pixels.frame_mut(), &resources.light_map);
    }
}

fn update_light(light: &Light, position: &Position, light_map: &mut LightMap, camera: &Camera) {
    // Offsetting the position by half a tile size to center the light in the tile
    let position_offset = Position::new(TILE_SIZE as f32 / 2.0, TILE_SIZE as f32 / 2.0);
    let screen_pos = camera.world_to_screen(*position + position_offset);
    let scaled_x = (screen_pos.x / light_map.scale as f32) as i32;
    let scaled_y = (screen_pos.y / light_map.scale as f32) as i32;
    let scaled_radius = (light.radius / light_map.scale as f32) as i32;
    let solid_center = (light.radius / 12.0 / light_map.scale as f32) as i32;

    for y in -scaled_radius..=scaled_radius {
        let world_y = scaled_y + y;
        if world_y < 0 || world_y >= light_map.height as i32 {
            continue;
        }

        for x in -scaled_radius..=scaled_radius {
            let world_x = scaled_x + x;
            if world_x < 0 || world_x >= light_map.width as i32 {
                continue;
            }

            let distance = ((x * x + y * y) as f32).sqrt();
            if distance <= scaled_radius as f32 {
                let intensity = if distance <= solid_center as f32 {
                    light.intensity
                } else {
                    let falloff_distance = distance - solid_center as f32;
                    let falloff_range = scaled_radius as f32 - solid_center as f32;
                    let t = (falloff_distance / falloff_range).clamp(0.0, 1.0);
                    let smoothed = (1.0 - (t * std::f32::consts::PI).cos()) * 0.5;
                    (1.0 - smoothed) * light.intensity
                };
                let idx = ((world_y * light_map.width as i32 + world_x) * 4) as usize;

                for i in 0..3 {
                    let current = light_map.buffer[idx + i] as f32 / 255.0;
                    let contribution = intensity * light.color[i];
                    let combined = (current + contribution).min(1.0);
                    light_map.buffer[idx + i] = (combined * 255.0) as u8;
                }
                light_map.buffer[idx + 3] = 255;
            }
        }
    }
}

pub struct LightUpdateSystem;

impl System for LightUpdateSystem {
    fn update(
        &self,
        world: &mut hecs::World,
        resources: &mut Resources,
        _: &mut Pixels,
        _: &Input,
        _: Duration,
    ) {
        resources.light_map.clear();

        for (_, (light, position)) in world.query::<(&Light, &Position)>().iter() {
            update_light(light, position, &mut resources.light_map, &resources.camera);
        }
    }
}

fn render_lighting(frame: &mut [u8], light_map: &LightMap) {
    let ambient_light = 0.1;

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let light_x = (x / light_map.scale) as usize;
            let light_y = (y / light_map.scale) as usize;
            let light_idx = (light_y * light_map.width as usize + light_x) * 4;
            let frame_idx = ((y * SCREEN_WIDTH + x) * 4) as usize;

            for i in 0..3 {
                let light_level = light_map.buffer[light_idx + i] as f32 / 255.0;
                let color = frame[frame_idx + i] as f32 / 255.0;
                let lit_color = color * (ambient_light + light_level);
                frame[frame_idx + i] = (lit_color * 255.0) as u8;
            }
            // Don't modify alpha
            frame[frame_idx + 3] = 255;
        }
    }
}
