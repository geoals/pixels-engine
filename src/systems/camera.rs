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

const DAMPING: f32 = 20.0;

impl System for CameraFollowSystem {
    fn update(
        &self,
        world: &mut hecs::World,
        resources: &mut Resources,
        _: &mut Pixels,
        _: &Input,
        delta_time: Duration,
    ) {
        let camera = &mut resources.camera;
        let offset = Vec2::new((TILE_SIZE / 2) as f32, TILE_SIZE as f32 / 2.0);
        let dead_zone = TILE_SIZE as f32 * 2.5;

        for (_, position) in world.query_mut::<With<&Position, &Player>>() {
            let target_pos = *position + offset;
            let current_pos = camera.position();

            // Calculate distance from camera to target
            let delta = target_pos - current_pos;
            let distance = (delta.x.powi(2) + delta.y.powi(2)).sqrt();

            // Calculate damping factor based on distance from player
            let base_damping = if distance > dead_zone {
                DAMPING // Use full damping when beyond dead zone
            } else {
                DAMPING * 0.5 // Use reduced damping within dead zone
            };

            let damping_factor = 1.0 - (-base_damping * delta_time.as_secs_f32()).exp();
            let movement = delta * damping_factor;

            camera.set_position(current_pos + movement);
        }
    }
}
