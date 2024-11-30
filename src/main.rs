use std::thread;
use std::time::Instant;

use application::Application;
use pixels::Error;
use pixels_engine::fps_counter::FpsCounter;
use pixels_engine::{SCALE_FACTOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod application;

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

    let mut application = Application::new(&window);

    let mut fps_counter = FpsCounter::new(240);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } if !application.process_input_events(&event) => {
            match event {
                WindowEvent::Resized(size) => {
                    application.resize(size.width, size.height);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => {}
            }
        }
        Event::RedrawRequested(_) => {
            let start_time = Instant::now();

            application.clear();
            application.update();
            application.draw();

            let elapsed_time = start_time.elapsed();
            let sleep_time = fps_counter.calculate_sleep_time(elapsed_time);
            fps_counter.update_and_print(elapsed_time + sleep_time);

            thread::sleep(sleep_time);
            application.delta_time = elapsed_time + sleep_time;
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
