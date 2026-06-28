// main.rs
mod rawdaug;
mod util;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

#[derive(Default)]
struct App {
    window: Option<Window>,
    rd: Option<rawdaug::RDObject>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("rawdaug"))
            .unwrap();

        let display_handle = window.display_handle().unwrap();
        let window_handle = window.window_handle().unwrap();
        self.rd = Some(rawdaug::RDObject::new(&display_handle.as_raw(), &window_handle.as_raw(), &window).expect("failed"));
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

use log::{info, error, debug};
fn main() {
    util::log::init_log();
    log::info!("Welcome to paris!");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}