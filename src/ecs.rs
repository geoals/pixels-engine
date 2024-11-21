use crate::resource::Resource;
use crate::systems::System;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    // Same as before
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    // Same as before
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        // `&mut self` already guarantees we have
        // exclusive access to self so can use `get_mut` here
        // which avoids any runtime checks.
        self.get_mut().push(None)
    }
}

#[derive(Default)]
pub struct World {
    entities_count: usize,
    components: Vec<Box<dyn ComponentVec>>,
    systems: Vec<Box<dyn System>>,
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.components.iter_mut() {
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

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.components.iter_mut() {
            // The `downcast_mut` type here is changed to `RefCell<Vec<Option<ComponentType>>`
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // add a `get_mut` here. Once again `get_mut` bypasses
                // `RefCell`'s runtime checks if accessing through a `&mut` reference.
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        // Here we create a `RefCell` before inserting into `component_vecs`
        self.components
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                // Here we use `borrow_mut`.
                // If this `RefCell` is already borrowed from this will panic.
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    pub fn add_resource(&mut self, resource: impl Any) {
        self.resources.add(resource);
    }

    pub fn get_resource<T: 'static>(&self) -> Option<Ref<'_, T>> {
        self.resources.get_ref()
    }

    pub fn get_resource_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Position(f32, f32);

    #[derive(Debug, PartialEq)]
    struct Velocity(f32, f32);

    #[test]
    fn test_multiple_entities() {
        let mut world = World::new();

        // Create first entity
        let e1 = world.new_entity();
        world.add_component_to_entity(e1, Position(0.0, 0.0));

        // Create second entity
        let e2 = world.new_entity();
        world.add_component_to_entity(e2, Position(1.0, 1.0));

        // Verify both entities have components
        let positions = world.borrow_components_mut::<Position>().unwrap();
        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0], Some(Position(0.0, 0.0)));
        assert_eq!(positions[1], Some(Position(1.0, 1.0)));
    }

    #[test]
    fn test_multiple_component_types() {
        let mut world = World::new();

        // Create entity with multiple components
        let e1 = world.new_entity();
        world.add_component_to_entity(e1, Position(0.0, 0.0));
        world.add_component_to_entity(e1, Velocity(1.0, 1.0));

        // Create second entity with just position
        let e2 = world.new_entity();
        world.add_component_to_entity(e2, Position(2.0, 2.0));

        // Verify components
        let positions = world.borrow_components_mut::<Position>().unwrap();
        let velocities = world.borrow_components_mut::<Velocity>().unwrap();

        assert_eq!(positions.len(), 2);
        assert_eq!(velocities.len(), 2);

        assert_eq!(positions[0], Some(Position(0.0, 0.0)));
        assert_eq!(positions[1], Some(Position(2.0, 2.0)));
        assert_eq!(velocities[0], Some(Velocity(1.0, 1.0)));
        assert_eq!(velocities[1], None);
    }
}
