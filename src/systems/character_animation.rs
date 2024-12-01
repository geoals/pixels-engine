use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{AnimatedSprite, Movement},
    input::Input,
    resource::Resources,
};

use super::System;

pub struct CharacterAnimationSystem;

impl System for CharacterAnimationSystem {
    fn update(
        &self,
        hecs_world: &mut hecs::World,
        _: &mut Resources,
        _: &mut Pixels,
        input: &Input,
        delta_time: Duration,
    ) {
        for (_, (sprite, movement)) in hecs_world.query_mut::<(&mut AnimatedSprite, &Movement)>() {
            // BUG: letting go of input should not stop animation immediately, let two frames play
            if movement.is_moving || input.x() != 0 || input.y() != 0 {
                sprite.frame_time += delta_time.as_secs_f32();
                if sprite.frame_time >= animation_frame_duration(input) {
                    let frames = sprite.get_sprite_frames(&movement.direction, true);
                    sprite.current_animation_frame =
                        (sprite.current_animation_frame + 1) % frames.len();
                    sprite.frame_time -= animation_frame_duration(input);
                }
            } else {
                sprite.current_animation_frame = 0; // Reset to first frame when not moving
                sprite.frame_time = 0.0;
            }
        }
    }
}

fn animation_frame_duration(input: &Input) -> f32 {
    if input.shift() {
        0.075
    } else {
        0.125
    }
}
