use std::time::Instant;

use image::RgbaImage;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use player::Gengar;
use crate::draw::draw_grid;

use crate::input::Input;

mod input;
mod draw;
mod player;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 512;
pub const GRID_SIZE: u32 = 64;

struct World {
    entities: Vec<Box<dyn Render>>,
    input: Input,
    pixels: Pixels,
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
                Box::new(Gengar::new("./assets/gengar-64.png", 0, 0)),
                Box::new(Obstruction::new(2, 2, GRID_SIZE as i32)),
            ],
            input: Input::new(),
            pixels
        }
    }

    fn update(&mut self) {
        for entity in &mut self.entities {
            entity.update(&self.input);
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
            *byte = 0;
            // *byte = if i % 4 == 3 { 255 } else { 0 };
        }
    }
}

trait Render {
    fn draw(&self, frame: &mut [u8]) {
        if let Some(pixels) = self.pixels() {
            let image_width = pixels.width() as usize;
            let image_height = pixels.height() as usize;

            for (i, pixel) in pixels.chunks_exact(4).enumerate() {
                // Don't draw fully transparent pixels
                if pixel[3] == 0 {
                    continue;
                }
                let src_x = (i % image_width) as i32;
                let src_y = (i / image_height) as i32;

                let frame_offset = (((self.pos_y() + src_y) * WIDTH as i32 + (self.pos_x() + src_x)) * 4) as usize;

                if frame_offset > frame.len() || self.pos_x() + src_x >= WIDTH as i32 || self.pos_x() + src_x < 0 {
                    continue;
                }

                if let Some(dest_pixel) = frame.get_mut(frame_offset..frame_offset + 4) {
                    dest_pixel.copy_from_slice(pixel);
                }
            }
        }
    }

    // should all renderable things have update method?
    fn update(&mut self, input: &Input) {

    }

    fn pixels(&self) -> Option<&RgbaImage> {
        None
    }
    fn pos_x(&self) -> i32;
    fn pos_y(&self) -> i32;
}

struct Obstruction {
    grid_x: u32,
    grid_y: u32,
    size: i32,
}

impl Obstruction {
    fn new(grid_x: u32, grid_y: u32, size: i32) -> Self {
        Self { grid_x, grid_y, size }
    }
}

impl Render for Obstruction {
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;

            let inside_the_box = x >= self.pos_x()
                && x < self.pos_x() + self.size
                && y >= self.pos_y()
                && y < self.pos_y() + self.size;

            if inside_the_box {
                pixel.copy_from_slice(&[255, 0, 0, 255])
            }
        }
    }

    fn pos_x(&self) -> i32 {
        (self.grid_x * GRID_SIZE) as i32
    }

    fn pos_y(&self) -> i32 {
        (self.grid_y * GRID_SIZE) as i32
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

    // FPS things
    let mut last_frame_time = Instant::now();
    let mut frames = 0;
    let mut fps_timer = Instant::now();

    let mut world = World::new(&window);

    event_loop.run(move |event, _, control_flow| {
        // Update the last_frame_time to the current time for the next frame
        last_frame_time = Instant::now();

        match event {
            Event::WindowEvent { event, .. } if !world.input.process_events(&event) => {
                match event {
                    WindowEvent::Resized(size) => {
                        world.pixels().resize_surface(size.width, size.height).unwrap();
                    }
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                world.clear();
                world.update();
                world.draw();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }

        frames += 1;
        // Calculate FPS every second
        let elapsed_since_fps_update = fps_timer.elapsed();
        if elapsed_since_fps_update.as_secs() >= 1 {
            let fps = frames as f64 / elapsed_since_fps_update.as_secs() as f64;
            println!("FPS: {:.2}", fps);
            frames = 0;
            fps_timer = Instant::now();
        }
    });
}
