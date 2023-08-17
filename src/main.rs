use std::time::Instant;

use image::RgbaImage;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use crate::draw::draw_grid;

use crate::input::Input;

mod input;
mod draw;

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

struct Gengar {
    pixels: Box<RgbaImage>,
    /// Utility value used to determine correct movement when two movement keys are pressed at once
    first_direction: Direction,
    current_direction: Direction,
    pos_x: i32,
    pos_y: i32,
}

impl Gengar {
    fn new(path: &str, pos_x: i32, pos_y: i32) -> Self {
        let image = image::open(path).unwrap();
        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());
        Self { pixels, pos_x, pos_y, first_direction: Direction::None, current_direction: Direction::None }
    }

    fn is_on_grid(&self) -> bool {
        self.is_on_horizontal_grid() && self.is_on_vertical_grid()
    }

    fn is_on_vertical_grid(&self) -> bool {
        self.pos_x % GRID_SIZE as i32 == 0
    }

    fn is_on_horizontal_grid(&self) -> bool {
        self.pos_y % GRID_SIZE as i32 == 0
    }

    fn set_first_direction(&mut self, input: &Input) {
        if self.first_direction == Direction::None {
            if input.x() == 0 {
                if input.y() == -1 {
                    self.first_direction = Direction::Up;
                }
                if input.y() == 1 {
                    self.first_direction = Direction::Down;
                }
            }
            if input.y() == 0 {
                if input.x() == -1 {
                    self.first_direction = Direction::Left;
                }
                if input.x() == 1 {
                    self.first_direction = Direction::Right;
                }
            }
        }
    }
}

/// Pixels per frame TODO time based instead of frame based
const GENGAR_SPEED: i32 = 4;

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}



impl Render for Gengar {
    fn update(&mut self, input: &Input) {
        self.set_first_direction(input);

        if input.y() == 1 && self.is_on_vertical_grid() {
            self.current_direction = Direction::Down;
        }
        if input.y() == -1 && self.is_on_vertical_grid() {
            self.current_direction = Direction::Up;
        }
        if input.x() == 1 && !(self.first_direction == Direction::Right && input.y() != 0) && self.is_on_horizontal_grid() {
            self.current_direction = Direction::Right;
        }
        if input.x() == -1 && !(self.first_direction == Direction::Left && input.y() != 0) && self.is_on_horizontal_grid() {
            self.current_direction = Direction::Left;
        }

        if !input.is_movement_keys_pressed() && self.is_on_grid() {
            self.current_direction = Direction::None;
            self.first_direction = Direction::None;
        }

        match self.current_direction {
            Direction::Left => self.pos_x -= GENGAR_SPEED,
            Direction::Right => self.pos_x += GENGAR_SPEED,
            Direction::Up => self.pos_y -= GENGAR_SPEED,
            Direction::Down => self.pos_y += GENGAR_SPEED,
            Direction::None => (),
        }
    }

    fn pixels(&self) -> &RgbaImage {
        self.pixels.as_ref()
    }

    fn pos_x(&self) -> i32 {
        self.pos_x
    }

    fn pos_y(&self) -> i32 {
        self.pos_y
    }
}

trait Render {
    fn draw(&self, frame: &mut [u8]) {
        let image_width = self.pixels().width() as usize;
        let image_height = self.pixels().height() as usize;

        for (i, pixel) in self.pixels().chunks_exact(4).enumerate() {
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

    fn update(&mut self, input: &Input);
    // should all renderable things have update method?
    fn pixels(&self) -> &RgbaImage;
    fn pos_x(&self) -> i32;
    fn pos_y(&self) -> i32;
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
