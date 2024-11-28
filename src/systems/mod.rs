use std::time::Duration;

use pixels::Pixels;

use crate::{input::Input, resource::Resources};

pub mod animation;
pub mod camera;
pub mod debug_grid;
pub mod level_transition;
pub mod movement;
pub mod sprite_render;
pub mod tile_render;

pub trait System {
    fn update(
        &self,
        world: &mut hecs::World,
        resources: &mut Resources,
        pixels: &mut Pixels,
        input: &Input,
        delta_time: Duration,
    );
}
