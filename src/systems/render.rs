use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{AnimatedSprite, Movement, Position, SpriteType},
    ecs::World,
    input::Input,
    spritesheet::CharacterSpritesheet,
    HEIGHT, TILE_SIZE, WIDTH,
};

use super::System;

pub struct SpriteRenderSystem;

const SCALE: f32 = 4.0;

impl System for SpriteRenderSystem {
    fn update(&self, world: &World, pixels: &mut Pixels, input: &Input, _delta_time: Duration) {
        let sprite_components = world.borrow_components_mut::<AnimatedSprite>().unwrap();
        let position_components = world.borrow_components_mut::<Position>().unwrap();
        let movement_components = world.borrow_components_mut::<Movement>().unwrap();

        for i in 0..sprite_components.len() {
            if let (Some(sprite), Some(position), Some(movement)) = (
                &sprite_components[i],
                &position_components[i],
                &movement_components[i],
            ) {
                let sheet = match sprite.sprite_type {
                    SpriteType::Player => world.get_resource::<CharacterSpritesheet>().unwrap(),
                };
                let vertical_offset = -4.0 * SCALE; // TODO: no offset for other sprite types

                let is_moving = movement.is_moving || input.x() != 0 || input.y() != 0;

                let (sprite_x, sprite_y) = sprite.get_current_frame(&movement.direction, is_moving);

                sheet.0.draw_sprite_to_buffer(
                    sprite_x,
                    sprite_y,
                    pixels.frame_mut(),
                    WIDTH,
                    HEIGHT,
                    position.x as i32,
                    (position.y + vertical_offset) as i32,
                );
            }
        }
    }
}

pub struct DebugGridSystem;

impl System for DebugGridSystem {
    fn update(&self, _world: &World, pixels: &mut Pixels, _input: &Input, _delta_time: Duration) {
        let frame = pixels.frame_mut();

        // Draw horizontal lines
        for y in (1..HEIGHT).step_by(TILE_SIZE as usize) {
            for x in 0..WIDTH {
                let i = (4 * x + y * WIDTH * 4) as usize;
                frame[i] = 255; // R
                frame[i + 1] = 255; // G
                frame[i + 2] = 255; // B
                frame[i + 3] = 255; // A
            }
        }

        // Draw vertical lines
        for x in (1..WIDTH).step_by(TILE_SIZE as usize) {
            for y in 0..HEIGHT {
                let i = (4 * x + y * WIDTH * 4) as usize;
                frame[i] = 255; // R
                frame[i + 1] = 255; // G
                frame[i + 2] = 255; // B
                frame[i + 3] = 255; // A
            }
        }
    }
}
