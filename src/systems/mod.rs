use std::time::Duration;

use pixels::Pixels;

use crate::{input::Input, resource::Resources};

pub mod camera;
pub mod character_animation;
pub mod debug_grid;
pub mod level_transition;
pub mod light_control;
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

pub struct SystemTiming {
    accumulated_time: Duration,
    update_interval: Duration,
}

impl Default for SystemTiming {
    fn default() -> Self {
        Self {
            accumulated_time: Default::default(),
            update_interval: Duration::from_millis(1000 / 30),
        }
    }
}

#[derive(Default)]
pub struct SystemContainer {
    render_systems: Vec<Box<dyn System>>,
    fixed_update_systems: Vec<Box<dyn System>>,
    timing: SystemTiming,
}

impl SystemContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_render_system<T: System + 'static>(&mut self, system: T) {
        self.render_systems.push(Box::new(system));
    }

    pub fn add_update_system<T: System + 'static>(&mut self, system: T) {
        self.fixed_update_systems.push(Box::new(system));
    }

    pub fn get_render_systems(&self) -> &[Box<dyn System>] {
        &self.render_systems
    }

    pub fn get_update_systems(&self) -> &[Box<dyn System>] {
        &self.fixed_update_systems
    }

    pub fn should_update(&mut self, delta_time: Duration) -> bool {
        self.timing.accumulated_time += delta_time;
        if self.timing.accumulated_time >= self.timing.update_interval {
            self.timing.accumulated_time -= self.timing.update_interval;
            true
        } else {
            false
        }
    }

    pub fn get_fixed_delta_time(&self) -> Duration {
        self.timing.update_interval
    }
}

pub enum SystemType {
    Render,
    FixedUpdate,
}
