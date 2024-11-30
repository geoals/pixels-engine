use super::System;
use crate::{input::Input, resource::Resources, tile::TileAnimation};
use hecs::World;
use pixels::Pixels;
use std::time::Duration;

pub struct TileAnimationSystem;

impl System for TileAnimationSystem {
    fn update(
        &self,
        _world: &mut World,
        resources: &mut Resources,
        _pixels: &mut Pixels,
        _input: &Input,
        delta_time: Duration,
    ) {
        let current_level =
            resources.tilemap.levels.get_mut(&resources.current_level_id.0).unwrap();

        // Update all animated tiles
        for tile_data in current_level.tiles.values_mut() {
            if let Some(animation) = &mut tile_data.animation {
                animation.update(delta_time);
                // Update the tile's position to the current animation frame
                tile_data.tileset_position = animation.current_position();
            }
        }
    }
}
