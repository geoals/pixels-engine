use std::time::Duration;

use pixels::Pixels;

use crate::{input::Input, resource::Resources};

pub mod camera;
pub mod character_animation;
pub mod debug_grid;
pub mod level_transition;
pub mod light_render;
pub mod movement;
pub mod sprite_render;
pub mod tile_animation;
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

#[derive(Default)]
pub struct SystemContainer {
    systems: Vec<Box<dyn System>>,
}

impl SystemContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn all(&self) -> &[Box<dyn System>] {
        &self.systems
    }
}
