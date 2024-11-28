use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use crate::{
    camera::Camera,
    spritesheet::CharacterSpritesheet,
    systems::level_transition::ScreenTransition,
    tile::{CurrentLevelId, TileMap},
};

#[derive(Default)]
pub struct Resource {
    data: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

pub struct Resources {
    pub camera: Camera,
    pub character_spritesheet: CharacterSpritesheet,
    pub current_level_id: CurrentLevelId,
    pub tilemap: TileMap,
    pub screen_transition: ScreenTransition,
}

impl Resource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, resource: impl Any) {
        let type_id = resource.type_id();
        self.data.insert(type_id, RefCell::new(Box::new(resource)));
    }

    pub fn get_ref<T: 'static>(&self) -> Option<Ref<'_, T>> {
        let type_id = TypeId::of::<T>();
        self.data
            .get(&type_id)
            .map(|cell| Ref::map(cell.borrow(), |boxed| boxed.downcast_ref::<T>().unwrap()))
    }

    pub fn get_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id).map(|cell| {
            RefMut::map(cell.borrow_mut(), |boxed| {
                boxed.downcast_mut::<T>().unwrap()
            })
        })
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let boxed_any = self.data.remove(&type_id)?;
        let boxed_t = boxed_any.into_inner().downcast::<T>().ok()?;
        Some(*boxed_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct WorldWidth(pub f32);

    #[test]
    fn add_resource() {
        let resources = initialize_resource();
        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let borrowed = stored_resource.borrow();
        let extracted_world_width = borrowed.downcast_ref::<WorldWidth>().unwrap();
        assert_eq!(extracted_world_width.0, 100.0);
    }

    #[test]
    fn add_resource_twice() {
        let mut resources = Resource::new();
        resources.add(WorldWidth(100.0));
        resources.add(WorldWidth(200.0));
        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let borrowed = stored_resource.borrow();
        let extracted_world_width = borrowed.downcast_ref::<WorldWidth>().unwrap();
        assert_eq!(extracted_world_width.0, 200.0);
    }

    #[test]
    fn get_resource() {
        let resources = initialize_resource();
        if let Some(extracted_world_width) = resources.get_ref::<WorldWidth>() {
            assert_eq!(extracted_world_width.0, 100.0)
        };
    }

    #[test]
    fn get_resource_mut() {
        let resources = initialize_resource();
        // Modify the resource
        if let Some(mut width) = resources.get_mut::<WorldWidth>() {
            width.0 = 200.0;
        }
        // Verify the modification
        if let Some(width) = resources.get_ref::<WorldWidth>() {
            assert_eq!(width.0, 200.0);
        } else {
            panic!("Failed to get resource reference");
        };
    }

    #[test]
    fn get_nonexistent_resource() {
        let resources = Resource::new();
        assert!(resources.get_ref::<WorldWidth>().is_none());
        assert!(resources.get_mut::<WorldWidth>().is_none());
    }

    #[test]
    fn modify_multiple_times() {
        let resources = initialize_resource();
        // First modification
        if let Some(mut width) = resources.get_mut::<WorldWidth>() {
            width.0 = 200.0;
        }
        // Second modification
        if let Some(mut width) = resources.get_mut::<WorldWidth>() {
            width.0 *= 2.0;
        }
        // Verify final value
        if let Some(width) = resources.get_ref::<WorldWidth>() {
            assert_eq!(width.0, 400.0);
        } else {
            panic!("Failed to get resource reference");
        };
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
