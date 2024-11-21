pub mod camera;
pub mod components;
pub mod ecs;
pub mod fps_counter;
pub mod input;
pub mod movement_util;
pub mod resource;
pub mod spritesheet;
pub mod systems;
pub mod vec2;

pub use ecs::World;

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;
pub const TILE_SIZE: u32 = 16;
pub const SCALE_FACTOR: u32 = 4;
