use lazy_static::lazy_static;
use std::time::Duration;

use crate::input::Input;
use crate::movement_util::{Axis, Direction};

use crate::vec2::Vec2;
use crate::TILE_SIZE;

#[derive(Debug, Default)]
pub struct AnimatedSprite {
    pub sprite_type: SpriteType,
    pub current_animation_frame: usize,
    pub frame_time: f32,
}

pub mod sprite_positions {
    pub const PLAYER_IDLE_DOWN: &[(u32, u32)] = &[(26, 34)];
    pub const PLAYER_IDLE_UP: &[(u32, u32)] = &[(77, 34)];
    pub const PLAYER_IDLE_LEFT: &[(u32, u32)] = &[(111, 34)];
    pub const PLAYER_IDLE_RIGHT: &[(u32, u32)] = &[(145, 34)];

    pub const PLAYER_WALK_DOWN: &[(u32, u32)] = &[(26, 34), (9, 34), (26, 34), (43, 34)];
    pub const PLAYER_WALK_UP: &[(u32, u32)] = &[(77, 34), (60, 34), (77, 34), (94, 34)];
    pub const PLAYER_WALK_LEFT: &[(u32, u32)] = &[(111, 34), (128, 34)];
    pub const PLAYER_WALK_RIGHT: &[(u32, u32)] = &[(145, 34), (162, 34)];
}

impl AnimatedSprite {
    pub fn new(sprite_type: SpriteType) -> Self {
        Self {
            sprite_type,
            ..Default::default()
        }
    }

    pub fn get_sprite_frames(&self, direction: &Direction, is_moving: bool) -> &[(u32, u32)] {
        match (&self.sprite_type, direction, is_moving) {
            (SpriteType::Player, Direction::Down, false) => sprite_positions::PLAYER_IDLE_DOWN,
            (SpriteType::Player, Direction::Up, false) => sprite_positions::PLAYER_IDLE_UP,
            (SpriteType::Player, Direction::Left, false) => sprite_positions::PLAYER_IDLE_LEFT,
            (SpriteType::Player, Direction::Right, false) => sprite_positions::PLAYER_IDLE_RIGHT,
            (SpriteType::Player, Direction::Left, true) => sprite_positions::PLAYER_WALK_LEFT,
            (SpriteType::Player, Direction::Right, true) => sprite_positions::PLAYER_WALK_RIGHT,
            (SpriteType::Player, Direction::Up, true) => sprite_positions::PLAYER_WALK_UP,
            (SpriteType::Player, Direction::Down, true) => sprite_positions::PLAYER_WALK_DOWN,
        }
    }

    pub fn get_current_frame(&self, direction: &Direction, is_moving: bool) -> (u32, u32) {
        let frames = self.get_sprite_frames(direction, is_moving);
        frames[self.current_animation_frame % frames.len()]
    }
}

#[derive(Debug, Default)]
pub enum SpriteType {
    #[default]
    Player,
}

pub type Position = Vec2;

impl Position {
    pub fn at_tile(tile_x: i64, tile_y: i64) -> Self {
        Self::new(
            tile_x as f32 * crate::TILE_SIZE as f32,
            tile_y as f32 * crate::TILE_SIZE as f32,
        )
    }

    pub fn tile(&self) -> (i64, i64) {
        (
            (self.x / crate::TILE_SIZE as f32) as i64,
            (self.y / crate::TILE_SIZE as f32) as i64,
        )
    }

    pub fn aligned_tile(&self) -> Option<(i64, i64)> {
        // Check if position is exactly aligned with tile grid
        if self.x % crate::TILE_SIZE as f32 == 0.0 && self.y % crate::TILE_SIZE as f32 == 0.0 {
            Some(self.tile())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct Movement {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,

    // These are used to apply a delay before starting movement
    pub start_delay: Duration,
    // No delay when moving in this direction
    pub initial_direction: Direction,
    // How long since last movement
    pub idle_timer: Duration,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            ..Default::default()
        }
    }

    /// Returns false for no input or input in oppositing or perpendicular direction
    pub fn input_not_in_same_direction(&self, input: &Input) -> bool {
        match self.direction.axis() {
            Axis::Horizontal if input.x() * self.direction.x() <= 0 => true,
            Axis::Vertical if input.y() * self.direction.y() <= 0 => true,
            _ => false,
        }
    }
}

pub struct Player;
pub struct PlayerStartingPosition(pub Position);

pub struct Light {
    pub radius: f32,
    pub intensity: f32,
    pub color: [f32; 3], // RGB color of the light
}

impl Light {
    pub fn new(radius: f32, intensity: f32, color: [f32; 3]) -> Self {
        Self {
            radius,
            intensity,
            color,
        }
    }
}

pub struct FireSpell;

#[derive(Debug, Clone)]
pub struct SpellEffect {
    pub effect_type: SpellEffectType,
    pub current_frame: usize,
    pub frame_time: f32,
    pub is_finished: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SpellEffectType {
    Fireball,
}

impl SpellEffect {
    pub fn new(effect_type: SpellEffectType) -> Self {
        Self {
            effect_type,
            current_frame: 0,
            frame_time: 0.0,
            is_finished: false,
        }
    }

    pub fn get_sprite_frames(&self) -> &[(u32, u32)] {
        match self.effect_type {
            SpellEffectType::Fireball => FIREBALL_FRAMES.as_slice(),
        }
    }
}

lazy_static! {
    static ref FIREBALL_FRAMES: Vec<(u32, u32)> = (0..176).step_by(16).map(|x| (x, 0)).collect();
}
