use crate::resource::Resource;
use crate::systems::System;
use std::any::{Any, TypeId};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

type ComponentStorage<T> = RefCell<Vec<Option<T>>>;

#[derive(Default)]
pub struct World {
    entities_count: usize,
    components: HashMap<TypeId, Box<dyn Any>>,
    systems: Vec<Box<dyn System>>,
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.components.values_mut() {
            if let Some(vec) = component_vec.downcast_mut::<ComponentStorage<Box<dyn Any>>>() {
                vec.get_mut().push(None);
            }
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
        let storage = self.components.entry(TypeId::of::<T>()).or_insert_with(|| {
            let mut vec: Vec<Option<T>> = Vec::with_capacity(self.entities_count);
            vec.extend((0..self.entities_count).map(|_| None));
            Box::new(ComponentStorage::new(vec))
        });

        if let Some(storage) = storage.downcast_mut::<ComponentStorage<T>>() {
            storage.get_mut()[entity] = Some(component);
        }
    }

    pub fn borrow_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|component_vec| {
                component_vec
                    .downcast_ref::<ComponentStorage<T>>()
                    .map(|vec| vec.borrow_mut())
            })
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut()
    }

    /// Removes a resource of type `T` from the world and returns it.
    ///
    /// # Arguments
    /// * `self` - Mutable reference to the world
    /// * `T` - The type of resource to remove
    ///
    /// # Returns
    /// * `Some(T)` - The removed resource if it existed
    /// * `None` - If no resource of type `T` was found
    ///
    /// # Example
    /// ```
    /// use pixels_engine::World;
    /// let mut world = World::new();
    /// world.add_resource(42_u32);
    ///
    /// // Remove the resource
    /// let number = world.remove_resource::<u32>().unwrap();
    /// assert_eq!(number, 42);
    ///
    /// // Resource is no longer in the world
    /// assert!(world.remove_resource::<u32>().is_none());
    /// ```
    pub fn remove_resource<T: Any>(&mut self) -> Option<T> {
        self.resources.remove::<T>()
    }
}
