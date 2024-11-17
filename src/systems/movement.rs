use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{Movement, Position},
    ecs::World,
    input::Input,
    movement_util::{Axis, Direction, PositionExt},
    vec2::Vec2,
    TILE_SIZE,
};

use super::System;

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&self, world: &World, _pixels: &mut Pixels, input: &Input, delta_time: Duration) {
        let mut movement_components = world.borrow_components_mut::<Movement>().unwrap();
        let mut position_components = world.borrow_components_mut::<Position>().unwrap();
        let zip = movement_components
            .iter_mut()
            .zip(position_components.iter_mut());
        let iter =
            zip.filter_map(|(movement, position)| Some((movement.as_mut()?, position.as_mut()?)));

        let delta_time = delta_time.as_secs_f32();

        for (movement, position) in iter {
            let should_stop = match movement.direction.axis() {
                Axis::Horizontal if input.x() * movement.direction.x() <= 0 => true,
                Axis::Vertical if input.y() * movement.direction.y() <= 0 => true,
                _ => false,
            };

            if should_stop
                && self.will_reach_next_tile_in_next_update(
                    delta_time,
                    &position.0,
                    &movement.direction,
                    movement.speed,
                    movement.is_moving,
                )
            {
                movement.is_moving = false;
                self.snap_to_grid(position);
            }

            if Direction::from_vector(input.vector()).is_some() {
                movement.is_moving = true;
            }

            if !movement.is_moving || self.is_on_grid(&position.0) {
                match input.y() {
                    1 => {
                        movement.direction = Direction::Down;
                    }
                    -1 => {
                        movement.direction = Direction::Up;
                    }
                    _ => {}
                }
                match input.x() {
                    1 => {
                        movement.direction = Direction::Right;
                    }
                    -1 => {
                        movement.direction = Direction::Left;
                    }
                    _ => {}
                }
            }

            if movement.is_moving {
                self.apply_movement(
                    delta_time,
                    &mut position.0,
                    &movement.direction,
                    movement.speed,
                );
            }
        }
    }
}

impl MovementSystem {
    fn snap_to_grid(&self, position: &mut Position) {
        position.0.x = (position.0.x / TILE_SIZE as f32).round() * TILE_SIZE as f32;
        position.0.y = (position.0.y / TILE_SIZE as f32).round() * TILE_SIZE as f32;
    }

    fn will_reach_next_tile_in_next_update(
        &self,
        delta_time: f32,
        position: &Vec2,
        direction: &Direction,
        speed: f32,
        is_moving: bool,
    ) -> bool {
        if !is_moving {
            return false;
        }

        let current_tile = position.tile_coordinate();
        let movement_vector = direction.to_vector();
        let movement_step = movement_vector.mul(speed * delta_time);
        let next_position = position.add(movement_step);
        let next_tile = next_position.tile_coordinate();

        current_tile != next_tile
    }

    fn apply_movement(
        &self,
        delta_time: f32,
        position: &mut Vec2,
        direction: &Direction,
        speed: f32,
    ) {
        let movement_vector = direction.to_vector();
        let movement_step = movement_vector.mul(speed * delta_time);
        position.x += movement_step.x;
        position.y += movement_step.y;
    }

    fn is_on_horizontal_grid(&self, position: &Vec2) -> bool {
        position.y % TILE_SIZE as f32 == 0.0
    }

    fn is_on_vertical_grid(&self, position: &Vec2) -> bool {
        position.x % TILE_SIZE as f32 == 0.0
    }

    fn is_on_grid(&self, position: &Vec2) -> bool {
        self.is_on_horizontal_grid(position) && self.is_on_vertical_grid(position)
    }
}
