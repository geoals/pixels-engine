pub mod components;
pub mod draw;
pub mod ecs;
pub mod input;
pub mod movement_util;
mod resource;
pub mod systems;
pub mod vec2;

pub use ecs::World;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 576;
pub const TILE_SIZE: u32 = 64;
