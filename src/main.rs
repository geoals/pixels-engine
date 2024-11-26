use std::thread;
use std::time::{Duration, Instant};

use pixels::{Error, Pixels, SurfaceTexture};
use pixels_engine::camera::Camera;
use pixels_engine::components::{AnimatedSprite, Movement, Player, Position, SpriteType};
use pixels_engine::fps_counter::FpsCounter;
use pixels_engine::input::Input;
use pixels_engine::spritesheet::{CharacterSpritesheet, Spritesheet};
use pixels_engine::systems::animation::AnimationSystem;
use pixels_engine::systems::camera::CameraFollowSystem;
use pixels_engine::systems::collision::CollisionSystem;
use pixels_engine::systems::debug_grid::DebugGridSystem;
use pixels_engine::systems::movement::MovementSystem;
use pixels_engine::systems::sprite_render::SpriteRenderSystem;
use pixels_engine::systems::tile_render::TileRenderSystem;
use pixels_engine::tile::TileMap;
use pixels_engine::{ecs, World, SCALE_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

struct Application {
    world: ecs::World,
    input: Input,
    pixels: Pixels,
    delta_time: Duration,
}

impl Application {
    fn new(window: &Window) -> Self {
        let mut pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(
                size.width / SCALE_FACTOR,
                size.height / SCALE_FACTOR,
                &window,
            );
            Pixels::new(
                size.width / SCALE_FACTOR,
                size.height / SCALE_FACTOR,
                surface_texture,
            )
            .unwrap()
        };
        pixels.enable_vsync(true);

        let mut world = World::new();

        world.add_resource(CharacterSpritesheet(
            Spritesheet::new("./assets/characters_spritesheet.png", 16, 16).unwrap(),
        ));
        let tilemap = TileMap::load("./assets/world.ldtk").unwrap();
        let player_starting_position = tilemap.player_starting_position;
        world.add_resource(tilemap);
        world.add_resource(Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT));

        let player = world.new_entity();

        world.add_component_to_entity(player, AnimatedSprite::new(SpriteType::Player));
        world.add_component_to_entity(
            player,
            Position::new(player_starting_position.x, player_starting_position.y),
        );

        world.add_component_to_entity(player, Movement::new(48.0));
        world.add_component_to_entity(player, Player);

        world.add_system(MovementSystem);
        // world.add_system(CollisionSystem);
        world.add_system(AnimationSystem);
        world.add_system(CameraFollowSystem);
        if cfg!(feature = "debug") {
            world.add_system(DebugGridSystem);
        }
        world.add_system(TileRenderSystem);
        world.add_system(SpriteRenderSystem);

        Self {
            world,
            input: Input::new(),
            pixels,
            delta_time: Duration::new(0, 0),
        }
    }

    fn update(&mut self) {
        for system in self.world.systems() {
            system.update(&self.world, &mut self.pixels, &self.input, self.delta_time);
        }
    }

    fn draw(&mut self) {
        self.pixels.render().unwrap();
    }

    fn pixels(&mut self) -> &mut Pixels {
        &mut self.pixels
    }

    /// Clear the screen
    fn clear(&mut self) {
        for byte in self.pixels.frame_mut().iter_mut() {
            *byte = 0;
        }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(
            (SCREEN_WIDTH * SCALE_FACTOR) as f64,
            (SCREEN_HEIGHT * SCALE_FACTOR) as f64,
        );

        WindowBuilder::new()
            .with_title("PixelsEngine")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut world = Application::new(&window);

    let mut fps_counter = FpsCounter::new(240);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } if !world.input.process_events(&event) => match event {
            WindowEvent::Resized(size) => {
                world
                    .pixels()
                    .resize_surface(size.width, size.height)
                    .unwrap();
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        Event::RedrawRequested(_) => {
            let start_time = Instant::now();

            world.clear();
            world.update();
            world.draw();

            let elapsed_time = start_time.elapsed();
            let sleep_time = fps_counter.calculate_sleep_time(elapsed_time);
            fps_counter.update_and_print(elapsed_time + sleep_time);

            thread::sleep(sleep_time);
            world.delta_time = elapsed_time + sleep_time;
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
