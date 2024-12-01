use super::System;
use crate::{
    components::{Position, SpellEffect},
    input::Input,
    resource::Resources,
};
use hecs::World;
use pixels::Pixels;
use std::time::Duration;

pub struct SpellEffectRenderSystem;

impl System for SpellEffectRenderSystem {
    fn update(
        &self,
        world: &mut World,
        resources: &mut Resources,
        pixels: &mut Pixels,
        _: &Input,
        delta_time: Duration,
    ) {
        for (_, (effect, position)) in world.query_mut::<(&mut SpellEffect, &Position)>() {
            effect.frame_time += delta_time.as_secs_f32();

            if effect.frame_time >= 0.06 {
                let cloned_effect = effect.clone();
                let frames = cloned_effect.get_sprite_frames();
                effect.current_frame += 1;
                effect.frame_time = 0.0;

                if effect.current_frame >= frames.len() {
                    effect.is_finished = true;
                }
            }

            if !effect.is_finished {
                let frames = effect.get_sprite_frames();
                let (sprite_x, sprite_y) = frames[effect.current_frame % frames.len()];
                let screen_pos = resources.camera.world_to_screen(*position);

                resources.effects_spritesheet.0.draw_sprite_to_buffer(
                    sprite_x,
                    sprite_y,
                    pixels.frame_mut(),
                    screen_pos.x.round() as i32,
                    screen_pos.y.round() as i32,
                );
            }
        }

        let mut to_remove = Vec::new();
        for (entity, effect) in world.query::<&SpellEffect>().iter() {
            if effect.is_finished {
                to_remove.push(entity);
            }
        }

        for entity in to_remove {
            let _ = world.despawn(entity);
        }
    }
}
