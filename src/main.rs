use std::thread;
use std::time::{Duration, Instant};

use crate::draw::draw_grid;
use image::RgbaImage;
use pixels::{Error, Pixels, SurfaceTexture};
use player::Player;
use vec2::Vec2;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::input::Input;

mod draw;
mod input;
mod player;
mod vec2;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 576;
pub const TILE_SIZE: u32 = 64;

struct World {
    entities: Vec<Box<dyn Render>>,
    input: Input,
    pixels: Pixels,
    delta_time: Duration,
}

impl World {
    fn new(window: &Window) -> Self {
        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(size.width, size.height, surface_texture).unwrap()
        };

        Self {
            entities: vec![
                Box::new(Player::new()),
                Box::new(Obstruction::new(2, 2, TILE_SIZE as i32)),
                Box::new(Obstruction::new(3, 2, TILE_SIZE as i32)),
                Box::new(Obstruction::new(4, 2, TILE_SIZE as i32)),
            ],
            input: Input::new(),
            pixels,
            delta_time: Duration::new(0, 0),
        }
    }

    fn update(&mut self) {
        for entity in &mut self.entities {
            entity.update(&self.input, self.delta_time);
        }
    }

    fn draw(&mut self) {
        draw_grid(self.pixels().frame_mut()); // if debug
        for entity in &self.entities {
            entity.draw(self.pixels.frame_mut());
        }
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

// /// Blit a drawable to the pixel buffer.
// pub fn blit(screen: &mut [u8], dest: &Point, sprite: &S) {
//     assert!(dest.x + sprite.width() <= WIDTH);
//     assert!(dest.y + sprite.height() <= HEIGHT);
//
//     let pixels = sprite.pixels();
//     let width = sprite.width() * 4;
//
//     let mut s = 0;
//     for y in 0..sprite.height() {
//         let i = dest.x * 4 + dest.y * WIDTH * 4 + y * WIDTH * 4;
//
//         // Merge pixels from sprite into screen
//         let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
//         for (left, &right) in zipped {
//             if right > 0 {
//                 *left = right;
//             }
//         }
//
//         s += width;
//     }
// }

trait Render {
    fn draw(&self, frame: &mut [u8]) {
        let Some(pixels) = self.pixels() else { return };
        let image_width = pixels.width() as usize;
        // let image_height = pixels.height() as usize;

        for (i, pixel) in pixels.chunks_exact(4).enumerate() {
            // Don't draw fully transparent pixels
            if pixel[3] == 0 {
                continue;
            }
            let src_x = (i % image_width) as i32;
            let src_y = (i / image_width) as i32;
            // let src_y = (i / image_height) as i32;

            let frame_offset = (((self.position().y.floor() as i32 + src_y) * WIDTH as i32
                + (self.position().x.floor() as i32 + src_x))
                * 4) as usize;

            if frame_offset > frame.len()
                || self.position().x.floor() as i32 + src_x >= WIDTH as i32
                || self.position().x.floor() as i32 + src_x < 0
            {
                continue;
            }

            if let Some(dest_pixel) = frame.get_mut(frame_offset..frame_offset + 4) {
                dest_pixel.copy_from_slice(pixel);
            }
        }
    }

    // should all renderable things have update method?
    fn update(&mut self, input: &Input, delta_time: Duration);

    fn pixels(&self) -> Option<&RgbaImage> {
        None
    }

    fn position(&self) -> Vec2;
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

impl Render for Obstruction {
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as f32;
            let y = (i / WIDTH as usize) as f32;

            let inside_the_box = x >= self.position().x
                && x < self.position().x + self.size as f32
                && y >= self.position().y
                && y < self.position().y + self.size as f32;

            if inside_the_box {
                pixel.copy_from_slice(&[255, 0, 0, 255])
            }
        }
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn update(&mut self, input: &Input, delta_time: Duration) {}
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

    let mut world = World::new(&window);

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
