use pixels::Pixels;
use std::time::Duration;

use crate::{
    components::{Movement, Position},
    ecs::World,
    input::Input,
    movement_util::Direction,
    tile::TileMap,
    TILE_SIZE,
};

use super::System;

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&self, world: &World, _pixels: &mut Pixels, _input: &Input, _delta_time: Duration) {
        let tilemap = world.get_resource::<TileMap>().unwrap();
        let mut movement_components = world.borrow_components_mut::<Movement>().unwrap();
        let mut position_components = world.borrow_components_mut::<Position>().unwrap();

        for (movement, position) in movement_components
            .iter_mut()
            .zip(position_components.iter_mut())
            .filter_map(|(m, p)| Some((m.as_mut()?, p.as_mut()?)))
        {
            // First, ensure we're aligned to the grid
            if !movement.is_moving {
                self.snap_to_grid(position, tilemap.tilesize);
                continue;
            }

            // Get current tile position
            let current_tile = position.tile();

            // Calculate target tile based on movement direction
            let (target_tile_x, target_tile_y) = match movement.direction {
                Direction::Up => (current_tile.0, current_tile.1 - 1),
                Direction::Down => (current_tile.0, current_tile.1 + 1),
                Direction::Left => (current_tile.0 - 1, current_tile.1),
                Direction::Right => (current_tile.0 + 1, current_tile.1),
            };
            dbg!(current_tile);
            dbg!(target_tile_x, target_tile_y);

            // Check if the target tile exists and is traversable
            let can_move = if let Some(tile) = tilemap.tiles.get(&(target_tile_x, target_tile_y)) {
                tile.traversable
            } else {
                false
            };

            if !can_move {
                // If we can't move, stop movement and snap back to the last valid tile
                movement.is_moving = false;
                self.snap_to_grid(position, tilemap.tilesize);
            }
        }
    }
}
impl CollisionSystem {
    fn snap_to_grid(&self, position: &mut Position, tilesize: u32) {
        position.x = (position.x / tilesize as f32).round() * tilesize as f32;
        position.y = (position.y / tilesize as f32).round() * tilesize as f32;
    }
}
