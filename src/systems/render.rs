use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{Position, Sprite},
    ecs::World,
    input::Input,
    WIDTH,
};

use super::System;

pub struct RenderSystem;

impl System for RenderSystem {
    fn update(&self, world: &World, pixels: &mut Pixels, _input: &Input, _delta_time: Duration) {
        let sprite_components = world.borrow_components_mut::<Sprite>().unwrap();
        let position_components = world.borrow_components_mut::<Position>().unwrap();
        let zip = sprite_components.iter().zip(position_components.iter());
        let iter = zip.filter_map(|(sprite, position)| {
            if let (Some(sprite), Some(position)) = (sprite, position) {
                Some((sprite, position))
            } else {
                None
            }
        });

        for (sprite, position) in iter {
            self.draw(pixels.frame_mut(), sprite, position);
        }
    }
}

impl RenderSystem {
    fn draw(&self, frame: &mut [u8], sprite: &Sprite, position: &Position) {
        let sprite_pixels = sprite.0.as_ref();
        let image_width = sprite_pixels.width() as usize;
        // let image_height = pixels.height() as usize;

        for (i, pixel) in sprite_pixels.chunks_exact(4).enumerate() {
            // Don't draw fully transparent pixels
            if pixel[3] == 0 {
                continue;
            }
            let src_x = (i % image_width) as i32;
            let src_y = (i / image_width) as i32;
            // let src_y = (i / image_height) as i32;

            let frame_offset = (((position.y.floor() as i32 + src_y) * WIDTH as i32
                + (position.x.floor() as i32 + src_x))
                * 4) as usize;

            if frame_offset > frame.len()
                || position.x.floor() as i32 + src_x >= WIDTH as i32
                || position.x.floor() as i32 + src_x < 0
            {
                continue;
            }

            if let Some(dest_pixel) = frame.get_mut(frame_offset..frame_offset + 4) {
                dest_pixel.copy_from_slice(pixel);
            }
        }
    }
}
