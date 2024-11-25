use crate::{
    camera::Camera,
    components::{AnimatedSprite, Player, Position},
    input::Input,
    vec2::Vec2,
    World, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};
use pixels::Pixels;
pub use std::time::Duration;

use super::System;

pub struct CameraFollowSystem;

impl System for CameraFollowSystem {
    fn update(&self, world: &World, _pixels: &mut Pixels, _input: &Input, _delta_time: Duration) {
        let mut camera = world.get_resource_mut::<Camera>().unwrap();
        let sprite_components = world.borrow_components_mut::<AnimatedSprite>().unwrap();
        let position_components = world.borrow_components_mut::<Position>().unwrap();
        let player_components = world.borrow_components_mut::<Player>().unwrap();

        for i in 0..sprite_components.len() {
            if let (Some(_sprite), Some(player_position), Some(_player)) = (
                &sprite_components[i],
                &position_components[i],
                &player_components[i],
            ) {
                camera.set_position(
                    *player_position + Vec2::new(TILE_SIZE as f32, (TILE_SIZE / 2) as f32),
                );
            }
        }
    }
}
