use std::time::Duration;

use pixels::Pixels;

use crate::{
    components::{Movement, Position},
    input::Input,
    movement_util::{Direction, PositionExt},
    resource::Resources,
    tile::{CurrentLevelId, TileMap},
    vec2::Vec2,
    TILE_SIZE,
};

use super::System;

pub struct MovementSystem;

const MOVEMENT_DELAY: Duration = Duration::from_millis(100);

// Simplify prop passing
struct MovementContext<'a> {
    position: &'a mut Position,
    movement: &'a mut Movement,
    tilemap: &'a TileMap,
    delta_time: Duration,
    input: &'a Input,
    current_level_id: &'a CurrentLevelId,
}

impl System for MovementSystem {
    fn update(
        &self,
        world: &mut hecs::World,
        resources: &mut Resources,
        _pixels: &mut Pixels,
        input: &Input,
        delta_time: Duration,
    ) {
        for (_, (position, movement)) in world.query_mut::<(&mut Position, &mut Movement)>() {
            let mut ctx = MovementContext {
                position,
                movement,
                tilemap: &resources.tilemap,
                delta_time,
                input,
                current_level_id: &resources.current_level_id,
            };
            handle_movement(&mut ctx);
        }
    }
}

fn handle_movement(ctx: &mut MovementContext) {
    if ctx.movement.input_not_in_same_direction(ctx.input)
        && will_reach_next_tile_in_next_update(ctx)
    {
        ctx.movement.is_moving = false;
        snap_to_grid(ctx.position);

        if ctx.input.none() {
            ctx.movement.start_delay = Duration::ZERO;
            ctx.movement.idle_timer = Duration::ZERO;
        }
    }

    // Make sure the initial direction is updated when you are standing still
    // but changing direction without moving
    if ctx.input.none()
        && !ctx.movement.is_moving
        && ctx.movement.idle_timer >= Duration::from_millis(50)
    {
        ctx.movement.initial_direction = ctx.movement.direction;
        ctx.movement.start_delay = Duration::ZERO;
    }

    if let Some(current_direction) = ctx.input.current_direction() {
        // No delay if input matches initial direction
        if ctx.movement.initial_direction == current_direction
            || ctx.movement.start_delay >= MOVEMENT_DELAY
        {
            ctx.movement.is_moving = true;
        } else {
            // Apply start delay
            ctx.movement.start_delay += ctx.delta_time;
        }

        // Changing directions
        if is_on_grid(ctx.position) {
            ctx.movement.direction = current_direction;
        }
    };

    if is_traversable(ctx) && ctx.movement.is_moving {
        apply_movement(ctx);
    } else {
        ctx.movement.is_moving = false;
        ctx.movement.idle_timer += ctx.delta_time;
        snap_to_grid(ctx.position);
    }
}

fn snap_to_grid(position: &mut Position) {
    position.x = (position.x / TILE_SIZE as f32).round() * TILE_SIZE as f32;
    position.y = (position.y / TILE_SIZE as f32).round() * TILE_SIZE as f32;
}

fn will_reach_next_tile_in_next_update(ctx: &MovementContext) -> bool {
    if !ctx.movement.is_moving {
        return false;
    }

    let current_tile = ctx.position.tile_coordinate();
    let next_tile = next_position(ctx).tile_coordinate();
    current_tile != next_tile
}

fn apply_movement(ctx: &mut MovementContext) {
    *ctx.position = next_position(ctx);
}

fn is_on_grid(position: &Vec2) -> bool {
    (position.y % TILE_SIZE as f32 == 0.0) && (position.x % TILE_SIZE as f32 == 0.0)
}

/// Check if the tile at the leading edge of movement will be traversable after the next update.
/// For right/down movement, checks the far edge of the sprite since position represents the top-left corner.
fn is_traversable(ctx: &MovementContext) -> bool {
    let next_position = next_position(ctx);
    let collision_pos = match ctx.movement.direction {
        Direction::Right => Vec2::new(next_position.x + TILE_SIZE as f32, next_position.y),
        Direction::Down => Vec2::new(next_position.x, next_position.y + TILE_SIZE as f32),
        _ => next_position,
    };
    let collision_tile = collision_pos.tile_coordinate();

    ctx.tilemap.get_level(ctx.current_level_id).tiles[&(collision_tile.0, collision_tile.1)]
        .traversable
}

fn next_position(ctx: &MovementContext) -> Vec2 {
    let movement_speed_multiplier = if ctx.input.shift() { 3.0 } else { 1.0 };
    let movement_vector = ctx.movement.direction.to_vector();
    let movement_step = movement_vector
        * ctx.movement.speed
        * ctx.delta_time.as_secs_f32()
        * movement_speed_multiplier;
    *(ctx.position) + movement_step
}
