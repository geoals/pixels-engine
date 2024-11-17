use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{Movement, Position},
    ecs::World,
    input::Input,
    movement_util::{Direction, PositionExt},
    vec2::Vec2,
    TILE_SIZE,
};

use super::System;

pub struct MovementSystem;

const MOVEMENT_DELAY: f32 = 0.15;

// Simplify prop passing
struct MovementContext<'a> {
    position: &'a mut Position,
    movement: &'a mut Movement,
    delta_time: f32,
    input: &'a Input,
}

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
            let mut ctx = MovementContext {
                position,
                movement,
                delta_time,
                input,
            };
            self.handle_movement(&mut ctx)
        }
    }
}

impl MovementSystem {
    fn handle_movement(&self, ctx: &mut MovementContext) {
        if ctx.movement.is_moving {
            self.apply_movement(ctx);
        }

        if ctx.movement.input_not_in_same_direction(ctx.input)
            && self.will_reach_next_tile_in_next_update(ctx)
        {
            ctx.movement.is_moving = false;
            self.snap_to_grid(ctx.position);

            if ctx.input.none() {
                ctx.movement.start_delay = 0.0;
            }
        }

        // Make sure the initial direction is updated when you are standing still
        // but changing direction without moving
        // BUG: start_delay is not reset when changing direction without moving
        // which means you can only rotate your character a few times before moving
        // can possibly solved with adding an idle_timer
        if ctx.input.none() && ctx.movement.initial_direction != ctx.movement.direction {
            ctx.movement.initial_direction = ctx.movement.direction;
        }

        let Some(input_direction) = Direction::from_vector(ctx.input.vector()) else {
            return;
        };

        // No delay if input matches initial direction
        if ctx.movement.initial_direction == input_direction
            || ctx.movement.start_delay >= MOVEMENT_DELAY
        {
            ctx.movement.is_moving = true;
        } else {
            // Apply start delay
            ctx.movement.start_delay += ctx.delta_time;
        }

        // Changing directions
        if self.is_on_grid(ctx.position) {
            ctx.movement.direction = input_direction;
        }
    }

    fn snap_to_grid(&self, position: &mut Position) {
        position.x = (position.x / TILE_SIZE as f32).round() * TILE_SIZE as f32;
        position.y = (position.y / TILE_SIZE as f32).round() * TILE_SIZE as f32;
    }

    fn will_reach_next_tile_in_next_update(&self, ctx: &MovementContext) -> bool {
        if !ctx.movement.is_moving {
            return false;
        }

        let current_tile = ctx.position.tile_coordinate();
        let movement_vector = ctx.movement.direction.to_vector();
        let movement_step = movement_vector * ctx.movement.speed * ctx.delta_time;
        let next_position = *(ctx.position) + movement_step;
        let next_tile = next_position.tile_coordinate();

        current_tile != next_tile
    }

    fn apply_movement(&self, ctx: &mut MovementContext) {
        let movement_vector = ctx.movement.direction.to_vector();
        let movement_step = movement_vector * ctx.movement.speed * ctx.delta_time;
        *ctx.position += movement_step;
    }

    fn is_on_grid(&self, position: &Vec2) -> bool {
        (position.y % TILE_SIZE as f32 == 0.0) && (position.x % TILE_SIZE as f32 == 0.0)
    }
}
