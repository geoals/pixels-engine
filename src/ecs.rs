// based on https://ianjk.com/ecs-in-rust/

use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

use crate::systems::System;

#[derive(Default)]
pub struct World {
    entities_count: usize,
    components: HashMap<TypeId, Box<dyn ComponentVec>>,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.components.values_mut() {
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
        let type_id = TypeId::of::<ComponentStorage<T>>();

        if let Some(component_vec) = self.components.get_mut(&type_id) {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<ComponentStorage<T>>()
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
        self.components
            .insert(type_id, Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        let type_id = TypeId::of::<ComponentStorage<T>>();

        self.components.get(&type_id).and_then(|component_vec| {
            component_vec
                .as_any()
                .downcast_ref::<ComponentStorage<T>>()
                .map(|component_vec| component_vec.borrow_mut())
        })
    }
}

type ComponentStorage<T> = RefCell<Vec<Option<T>>>;

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentVec for ComponentStorage<T> {
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
