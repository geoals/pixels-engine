use image::{DynamicImage, RgbaImage};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BOX_SIZE: i32 = 64;

struct World {
    square: Square,
}

impl World {
    fn new() -> Self {
        Self {
            square: Square { pos_x: 24, pos_y: 16, velocity_x: 1, velocity_y: 1 }
        }
    }

    fn update(&mut self) {
        self.square.update();
    }

    fn draw(&self, frame: &mut [u8]) {
        self.square.draw(frame);
    }
}

struct Square {
    pos_x: i32,
    pos_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Square {
    fn update(&mut self) {
        if self.pos_x <= 0 || self.pos_x + BOX_SIZE > WIDTH as i32 {
            self.velocity_x *= -1;
        }
        if self.pos_y <= 0 || self.pos_y + BOX_SIZE > HEIGHT as i32 {
            self.velocity_y *= -1;
        }

        self.pos_x += self.velocity_x;
        self.pos_y += self.velocity_y;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i32;
            let y = (i / WIDTH as usize) as i32;

            let inside_the_box = x >= self.pos_x
                && x < self.pos_x + BOX_SIZE
                && y >= self.pos_y
                && y < self.pos_y + BOX_SIZE;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

struct Image {
    pixels: Box<RgbaImage>,
}

impl Image {
    fn new(path: &str) -> Self {
        let image = image::open(path).unwrap();
        let pixels = Box::new(image.as_rgba8().unwrap().to_owned());
        Self { pixels }
    }

    fn draw(&self, frame: &mut [u8], dest_x: usize, dest_y: usize) {
        let image_width = self.pixels.width() as usize;
        let image_height = self.pixels.height() as usize;

        for (i, pixel) in self.pixels.chunks_exact(4).enumerate() {
            let src_x = i % image_width;
            let src_y = i / image_height;

            let frame_offset = (dest_y + src_y) * (WIDTH as usize) + (dest_x + src_x); // TODO dynamic width
            let pixel_offset = src_y * image_width + src_x;

            if let Some(dest_pixel) = frame.get_mut(frame_offset..frame_offset + 4) {
                if let Some(src_pixel) = self.pixels.get(pixel_offset * 4..(pixel_offset + 1) * 4) {
                    dest_pixel.copy_from_slice(src_pixel);
                }
            }
            // if pixel_offset < self.pixels.len() && frame_offset + 3 < frame.len() {
            //     if let Some(dest_pixel) = frame.get_mut(frame_offset..frame_offset + 4) {
            //         dest_pixel.copy_from_slice(&pixel[0..4]);
            //     }
            // }
        }
    }
    // fn pixels(&self) -> &RgbaImage { &self.pixels }
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

    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surface_texture)?
    };
    let mut world = World::new();

    let image = Image::new("./assets/gengar.png");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(size) => {
                        pixels.resize_surface(size.width, size.height).unwrap();
                    }
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                world.update();
                // world.draw(pixels.frame_mut());
                image.draw(pixels.frame_mut(), 0, 0);
                pixels.render().unwrap();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
