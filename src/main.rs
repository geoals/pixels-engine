use std::thread;
use std::time::{Duration, Instant};

use crate::draw::draw_grid;
use ecs::{Movement, Position, Sprite, World};
use pixels::{Error, Pixels, SurfaceTexture};
use systems::movement::MovementSystem;
use systems::render::RenderSystem;
use vec2::Vec2;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::input::Input;

mod draw;
mod ecs;
mod input;
mod movement_util;
mod systems;
mod vec2;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 576;
pub const TILE_SIZE: u32 = 64;

struct Application {
    world: ecs::World,
    input: Input,
    pixels: Pixels,
    delta_time: Duration,
}

impl Application {
    fn new(window: &Window) -> Self {
        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(size.width, size.height, surface_texture).unwrap()
        };

        let mut world = World::new();
        let player = world.new_entity();

        let image_path = "./assets/gengar-64.png";
        let image = image::open(image_path).unwrap();
        let player_sprite = Box::new(image.as_rgba8().unwrap().to_owned());
        world.add_component_to_entity(player, Sprite(player_sprite));
        world.add_component_to_entity(player, Position(Vec2::ZERO));
        world.add_component_to_entity(player, Movement::default());
        world.add_system(RenderSystem);
        world.add_system(MovementSystem);

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
        draw_grid(self.pixels().frame_mut()); // if debug
        self.pixels.render().unwrap();
    }

    fn pixels(&mut self) -> &mut Pixels {
        &mut self.pixels
    }

    /// Clear the screen
    fn clear(&mut self) {
        for (i, byte) in self.pixels.frame_mut().iter_mut().enumerate() {
            // *byte = 0;
            *byte = if i % 4 == 3 { 255 } else { 0 };
        }
    }
}

struct Obstruction {
    position: Vec2,
    size: i32,
}

impl Obstruction {
    fn new(grid_x: u32, grid_y: u32, size: i32) -> Self {
        Self {
            position: Vec2::new(
                grid_x as f32 * TILE_SIZE as f32,
                grid_y as f32 * TILE_SIZE as f32,
            ),
            size,
        }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("PixelsEngine")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut world = Application::new(&window);

    // Define the target FPS and calculate the desired frame interval
    let target_fps = 30;
    let frame_interval = Duration::from_micros(1_000_000 / target_fps);
    let mut last_time_for_fps_print = Instant::now();

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

            let end_time = Instant::now();
            let elapsed_time = end_time - start_time;
            let sleep_time = if elapsed_time < frame_interval {
                frame_interval - elapsed_time
            } else {
                Duration::ZERO
            };

            last_time_for_fps_print += elapsed_time;

            if last_time_for_fps_print.elapsed().as_millis() > 500 {
                dbg!(world.delta_time);
                last_time_for_fps_print = Instant::now();
            }

            thread::sleep(sleep_time);
            world.delta_time = elapsed_time + sleep_time;
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
