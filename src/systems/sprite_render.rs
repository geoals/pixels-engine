use std::time::{Duration, Instant};

use pixels::Pixels;

use crate::{
    camera::Camera,
    components::{AnimatedSprite, Movement, Position, SpriteType},
    ecs::World,
    input::Input,
    movement_util::Direction,
    spritesheet::CharacterSpritesheet,
    HEIGHT, WIDTH,
};

use super::System;

pub struct SpriteRenderSystem;

const SCALE: f32 = 1.0;

impl System for SpriteRenderSystem {
    fn update(&self, world: &World, pixels: &mut Pixels, input: &Input, _delta_time: Duration) {
        let camera = world.get_resource::<Camera>().unwrap();
        let spritesheet = world.get_resource::<CharacterSpritesheet>().unwrap();

        let sprite_components = world.borrow_components_mut::<AnimatedSprite>().unwrap();
        let position_components = world.borrow_components_mut::<Position>().unwrap();
        let movement_components = world.borrow_components_mut::<Movement>().unwrap();

        let frame = pixels.frame_mut();

        for i in 0..sprite_components.len() {
            if let (Some(sprite), Some(position)) = (&sprite_components[i], &position_components[i])
            {
                let movement = movement_components[i].as_ref();
                self.draw_sprite(
                    sprite,
                    position,
                    movement,
                    &camera,
                    &spritesheet,
                    frame,
                    input,
                );
            }
        }
    }
}

impl SpriteRenderSystem {
    #[allow(clippy::too_many_arguments)]
    fn draw_sprite(
        &self,
        sprite: &AnimatedSprite,
        position: &Position,
        movement: Option<&Movement>,
        camera: &Camera,
        spritesheet: &CharacterSpritesheet,
        frame: &mut [u8],
        input: &Input,
    ) {
        if !camera.is_visible(*position) {
            return;
        }

        let sheet = match sprite.sprite_type {
            SpriteType::Player => spritesheet,
        };

        let (direction, is_moving) = if let Some(movement) = movement {
            (
                &movement.direction,
                movement.is_moving || input.x() != 0 || input.y() != 0,
            )
        } else {
            (&Direction::Down, false)
        };

        let vertical_offset = -4.0 * SCALE;
        let (sprite_x, sprite_y) = sprite.get_current_frame(direction, is_moving);
        let screen_pos = camera.world_to_screen(*position);

        let sprite_data = sheet.0.get_sprite(sprite_x, sprite_y).unwrap();
        sheet.0.draw_sprite_to_buffer(
            &sprite_data,
            frame,
            WIDTH,
            HEIGHT,
            screen_pos.x as i32,
            (screen_pos.y + vertical_offset) as i32,
        );
    }
}
