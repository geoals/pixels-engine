use crate::{
    components::{Player, Position},
    input::Input,
    resource::Resources,
    vec2::Vec2,
    TILE_SIZE,
};
use hecs::With;
use pixels::Pixels;
pub use std::time::Duration;

use super::System;

pub struct CameraFollowSystem;

impl System for CameraFollowSystem {
    fn update(
        &self,
        hecs_world: &mut hecs::World,
        resources: &mut Resources,
        _pixels: &mut Pixels,
        _input: &Input,
        _delta_time: Duration,
    ) {
        let camera = &mut resources.camera;
        let offset = Vec2::new(TILE_SIZE as f32, (TILE_SIZE / 2) as f32);

        for (_, position) in hecs_world.query_mut::<With<&Position, &Player>>() {
            camera.set_position(*position + offset);
        }
    }
}
