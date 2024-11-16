use std::time::Duration;

use pixels::Pixels;

use crate::{ecs::World, input::Input};

pub mod movement;
pub mod render;

pub trait System {
    fn update(&self, world: &World, pixels: &mut Pixels, input: &Input, delta_time: Duration);
}
