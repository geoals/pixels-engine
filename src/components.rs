use crate::movement_util::Direction;

use crate::vec2::Vec2;

#[derive(Debug, Default)]
pub struct Sprite {
    pub sprite_type: SpriteType,
    pub current_animation_frame: usize,
    pub frame_time: f32,
}

pub mod sprite_positions {
    pub const PLAYER_IDLE_DOWN: &[(u32, u32)] = &[(26, 34)];
    pub const PLAYER_IDLE_UP: &[(u32, u32)] = &[(77, 34)];
    pub const PLAYER_IDLE_LEFT: &[(u32, u32)] = &[(111, 34)];
    pub const PLAYER_IDLE_RIGHT: &[(u32, u32)] = &[(145, 34)];

    pub const PLAYER_WALK_DOWN: &[(u32, u32)] = &[
        (9, 34),  // frame 0
        (26, 34), // frame 1
        (43, 34), // frame 2
        (26, 34), // frame 3
    ];

    pub const PLAYER_WALK_UP: &[(u32, u32)] = &[(60, 34), (77, 34), (94, 34), (77, 34)];

    pub const PLAYER_WALK_LEFT: &[(u32, u32)] = &[(111, 34), (128, 34)];

    pub const PLAYER_WALK_RIGHT: &[(u32, u32)] = &[(145, 34), (162, 34)];
}

impl Sprite {
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

pub struct Movement {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            speed: 256.0,
            direction: Direction::Down,
            is_moving: false,
        }
    }
}
