use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{AnimatedSprite, Movement, Position, SpriteType},
    ecs::World,
    input::Input,
    spritesheet::CharacterSpritesheet,
    HEIGHT, WIDTH,
};

use super::System;

pub struct SpriteRenderSystem;

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

                let is_moving = movement.is_moving || input.x() != 0 || input.y() != 0;

                let (sprite_x, sprite_y) = sprite.get_current_frame(&movement.direction, is_moving);

                sheet.0.draw_sprite_to_buffer(
                    sprite_x,
                    sprite_y,
                    pixels.frame_mut(),
                    WIDTH,
                    HEIGHT,
                    position.x as u32,
                    position.y as u32,
                );
            }
        }
    }
}
