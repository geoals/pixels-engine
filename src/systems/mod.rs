use std::time::Duration;

use pixels::Pixels;

use crate::{ecs::World, input::Input};

pub mod animation;
pub mod camera;
pub mod debug_grid;
pub mod movement;
pub mod sprite_render;

pub trait System {
    fn update(&self, world: &World, pixels: &mut Pixels, input: &Input, delta_time: Duration);
}
