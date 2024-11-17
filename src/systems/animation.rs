use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{AnimatedSprite, Movement},
    input::Input,
    World,
};

use super::System;

pub struct AnimationSystem;

impl System for AnimationSystem {
    fn update(&self, world: &World, _pixels: &mut Pixels, input: &Input, delta_time: Duration) {
        let mut sprite_components = world.borrow_components_mut::<AnimatedSprite>().unwrap();
        let movement_components = world.borrow_components_mut::<Movement>().unwrap();

        const FRAME_DURATION: f32 = 0.15;

        for i in 0..sprite_components.len() {
            if let (Some(sprite), Some(movement)) = (
                sprite_components[i].as_mut(),
                movement_components[i].as_ref(),
            ) {
                // BUG: letting go of input should not stop animation immediately, let two frames
                // play
                if movement.is_moving || input.x() != 0 || input.y() != 0 {
                    sprite.frame_time += delta_time.as_secs_f32();
                    if sprite.frame_time >= FRAME_DURATION {
                        let frames = sprite.get_sprite_frames(&movement.direction, true);
                        sprite.current_animation_frame =
                            (sprite.current_animation_frame + 1) % frames.len();
                        sprite.frame_time -= FRAME_DURATION;
                    }
                } else {
                    sprite.current_animation_frame = 0; // Reset to first frame when not moving
                    sprite.frame_time = 0.0;
                }
            }
        }
    }
}
