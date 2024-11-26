use crate::{
    camera::Camera,
    components::{Player, Position},
    input::Input,
    vec2::Vec2,
    World, TILE_SIZE,
};
use pixels::Pixels;
pub use std::time::Duration;

use super::System;

pub struct CameraFollowSystem;

impl System for CameraFollowSystem {
    fn update(&self, world: &World, _pixels: &mut Pixels, _input: &Input, _delta_time: Duration) {
        let mut camera = world.get_resource_mut::<Camera>().unwrap();
        let position_components = world.borrow_components_mut::<Position>().unwrap();
        let player_components = world.borrow_components_mut::<Player>().unwrap();

        for i in 0..position_components.len() {
            if let (Some(player_position), Some(_player)) =
                (&position_components[i], &player_components[i])
            {
                camera.set_position(
                    *player_position + Vec2::new(TILE_SIZE as f32, (TILE_SIZE / 2) as f32),
                );
            }
        }
    }
}
