mod input;

use std::time::{Duration, Instant};
use image::{DynamicImage, RgbaImage};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::input::Input;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

struct World {
    image: Gengar,
    input: Input,
}

impl World {
    fn new() -> Self {
        Self {
            image: Gengar::new("./assets/gengar.png", 0, 0),
            input: Input::new(),
        }
    }

    fn update(&mut self) {
        self.image.update(&self.input);
    }

    fn draw(&self, frame: &mut [u8]) {
        self.image.draw(frame);
    }
}

struct Gengar {
    pixels: Box<RgbaImage>,
    pos_x: i32,
    pos_y: i32,
}

const GENGAR_SPEED: i32 = 3;

impl Gengar {
    fn update(&mut self, input: &Input) {
        self.pos_x += input.x() * GENGAR_SPEED;
        self.pos_y += input.y() * GENGAR_SPEED;
    }
}

impl Gengar {
    fn new(path: &str, pos_x: i32, pos_y: i32) -> Self {
        let image = image::open(path).unwrap();
        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());
        Self { pixels, pos_x, pos_y }
    }

    fn draw(&self, frame: &mut [u8]) {
        let image_width = self.pixels.width() as usize;
        let image_height = self.pixels.height() as usize;

        for (i, pixel) in self.pixels.chunks_exact(4).enumerate() {
            let src_x = (i % image_width) as i32;
            let src_y = (i / image_height) as i32;

            let frame_offset = (((self.pos_y + src_y) * WIDTH as i32 + (self.pos_x + src_x)) * 4) as usize;

            if let Some(dest_pixel) = frame.get_mut(frame_offset..frame_offset + 4) {
                dest_pixel.copy_from_slice(pixel);
            }
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

    // FPS things
    let mut last_frame_time = Instant::now();
    let mut frames = 0;
    let mut fps_timer = Instant::now();

    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Update the last_frame_time to the current time for the next frame
        last_frame_time = Instant::now();

        match event {
            Event::WindowEvent { event, .. } if !world.input.process_events(&event) => {
                match event {
                    WindowEvent::Resized(size) => {
                        pixels.resize_surface(size.width, size.height).unwrap();
                    }
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit },
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                world.update();
                world.draw(pixels.frame_mut());
                pixels.render().unwrap();
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
