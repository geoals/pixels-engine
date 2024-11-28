pub mod camera;
pub mod components;
pub mod fps_counter;
pub mod input;
pub mod ivec2;
pub mod movement_util;
pub mod resource;
pub mod spritesheet;
pub mod system_container;
pub mod systems;
pub mod tile;
pub mod vec2;

pub use system_container::SystemContainer;

pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 144;
pub const TILE_SIZE: u32 = 16;
pub const SCALE_FACTOR: u32 = 4;
