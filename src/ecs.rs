// based on https://ianjk.com/ecs-in-rust/

use std::any::Any;
use std::cell::{RefCell, RefMut};

use image::RgbaImage;

use crate::movement_util::Direction;
use crate::systems::System;
use crate::vec2::Vec2;

pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
    systems: Vec<Box<dyn System>>,
}
impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
            systems: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn systems(&self) -> &[Box<dyn System>] {
        &self.systems
    }

    pub fn add_component_to_entity<T: 'static>(&mut self, entity: usize, component: T) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<T>>>>()
            {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<T>> = Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        // Give this Entity the Component.
        new_component_vec[entity] = Some(component);
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

pub struct Sprite(pub Box<RgbaImage>);

pub struct Position(pub Vec2);

pub struct Movement {
    pub speed: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

impl Movement {
    pub fn default() -> Self {
        Self {
            speed: 256.0,
            direction: Direction::Down,
            is_moving: false,
        }
    }
}
