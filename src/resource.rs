use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct Resource {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, resource: impl Any) {
        let type_id = resource.type_id();
        self.data.insert(type_id, Box::new(resource));
    }

    pub fn get_ref<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id)?.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.data.get_mut(&type_id)?.downcast_mut()
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let boxed_any = self.data.remove(&type_id)?;
        let boxed_t = boxed_any.downcast::<T>().ok()?;
        Some(*boxed_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct WorldWidth(pub f32);

    #[test]
    fn add_resource() {
        let resources = initialize_resource();

        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();
        assert_eq!(extracted_world_width.0, 100.0);
    }

    #[test]
    fn add_resource_twice() {
        let mut resources = Resource::new();
        resources.add(WorldWidth(100.0));
        resources.add(WorldWidth(200.0));
        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();
        assert_eq!(extracted_world_width.0, 200.0);
    }

    #[test]
    fn get_resource() {
        let resources = initialize_resource();

        if let Some(extracted_world_width) = resources.get_ref::<WorldWidth>() {
            assert_eq!(extracted_world_width.0, 100.0)
        }
    }
    #[test]
    fn get_resource_mut() {
        let mut resources = initialize_resource();

        // Modify the resource
        if let Some(width) = resources.get_mut::<WorldWidth>() {
            width.0 = 200.0;
        }

        // Verify the modification
        let width = resources.get_ref::<WorldWidth>().unwrap();
        assert_eq!(width.0, 200.0);
    }

    #[test]
    fn get_nonexistent_resource() {
        let mut resources = Resource::new();
        assert!(resources.get_ref::<WorldWidth>().is_none());
        assert!(resources.get_mut::<WorldWidth>().is_none());
    }

    #[test]
    fn modify_multiple_times() {
        let mut resources = initialize_resource();

        // First modification
        if let Some(width) = resources.get_mut::<WorldWidth>() {
            width.0 = 200.0;
        }

        // Second modification
        if let Some(width) = resources.get_mut::<WorldWidth>() {
            width.0 *= 2.0;
        }

        // Verify final value
        let width = resources.get_ref::<WorldWidth>().unwrap();
        assert_eq!(width.0, 400.0);
    }

    #[test]
    fn remove_resource() {
        let mut resources = initialize_resource();
        assert!(resources.get_ref::<WorldWidth>().is_some());
        resources.remove::<WorldWidth>();
        assert!(resources.get_ref::<WorldWidth>().is_none());
    }

    fn initialize_resource() -> Resource {
        let mut resources = Resource::new();
        let world_width = WorldWidth(100.0);

        resources.add(world_width);
        resources
    }
}
