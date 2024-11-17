use pixels_engine::World;

#[test]
fn create_and_get_resource_immutably() {
    let world = initialize_world();
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 30);
}

#[test]
fn create_and_get_resource_mutably() {
    let mut world = initialize_world();

    // Modify the FPS
    {
        let fps = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 = 60;
    }

    // Verify the modification
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60);
}

#[test]
fn remove_existing_resource() {
    let mut world = World::new();
    world.add_resource(FpsResource(30));

    // Verify resource exists
    assert!(world.get_resource::<FpsResource>().is_some());

    // Remove the resource
    let removed_fps = world.remove_resource::<FpsResource>().unwrap();
    assert_eq!(removed_fps.0, 30);

    // Verify resource is gone
    assert!(world.get_resource::<FpsResource>().is_none());
}

#[test]
fn remove_nonexistent_resource() {
    let mut world = World::new();
    assert!(world.remove_resource::<FpsResource>().is_none());
}

#[test]
fn remove_and_read_resource() {
    let mut world = World::new();
    world.add_resource(FpsResource(30));

    // Remove the resource
    let removed_fps = world.remove_resource::<FpsResource>().unwrap();
    assert_eq!(removed_fps.0, 30);

    // Add a new resource of the same type
    world.add_resource(FpsResource(60));

    // Verify new resource
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60);
}

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FpsResource(30));
    world
}

struct FpsResource(pub u32);
