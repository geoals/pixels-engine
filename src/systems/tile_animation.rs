use super::System;
use crate::{input::Input, resource::Resources};
use hecs::World;
use pixels::Pixels;
use std::time::Duration;

pub struct TileAnimationSystem;

impl System for TileAnimationSystem {
    fn update(
        &self,
        _: &mut World,
        resources: &mut Resources,
        _: &mut Pixels,
        _: &Input,
        delta_time: Duration,
    ) {
        let current_level = resources.tilemap.current_level_mut();

        for tile_data in current_level.tiles.values_mut() {
            if let Some(animation) = &mut tile_data.animation {
                animation.update(delta_time);
                tile_data.tileset_position = animation.current_position();
            }
        }
    }
}
