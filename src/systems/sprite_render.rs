use std::time::Duration;

use pixels::Pixels;

use crate::{
    camera::Camera,
    components::{AnimatedSprite, Movement, Position, SpriteType},
    input::Input,
    movement_util::Direction,
    resource::{CharacterSpritesheet, Resources},
};

use super::System;

pub struct SpriteRenderSystem;

impl System for SpriteRenderSystem {
    fn update(
        &self,
        world: &mut hecs::World,
        resources: &mut Resources,
        pixels: &mut Pixels,
        input: &Input,
        _delta_time: Duration,
    ) {
        for (_, (sprite, position, movement)) in
            world.query_mut::<(&AnimatedSprite, &Position, &Movement)>()
        {
            let frame = pixels.frame_mut();

            draw_sprite(
                sprite,
                position,
                Some(movement),
                &resources.camera,
                &mut resources.character_spritesheet,
                frame,
                input,
            );
        }
    }
}

fn draw_sprite(
    sprite: &AnimatedSprite,
    position: &Position,
    movement: Option<&Movement>,
    camera: &Camera,
    spritesheet: &mut CharacterSpritesheet,
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

    let vertical_offset = -4.0;
    let (sprite_x, sprite_y) = sprite.get_current_frame(direction, is_moving);
    let screen_pos = camera.world_to_screen(*position);

    sheet.0.draw_sprite_to_buffer(
        sprite_x,
        sprite_y,
        frame,
        screen_pos.x.round() as i32,
        (screen_pos.y + vertical_offset).round() as i32,
    );
}
