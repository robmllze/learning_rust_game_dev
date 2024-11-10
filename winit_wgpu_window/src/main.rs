// Imports.
use winit::{
    event_loop::{ControlFlow, EventLoop}, // EventLoop is the core of event handling in winit.
};

mod app;
use crate::app::App;

mod state;

// Main.

fn main() {
    env_logger::init();
    // Initializes the logger for capturing runtime logs. Useful for debugging.
    let event_loop = EventLoop::new().unwrap();
    // EventLoop manages events and application state, coordinating with the OS.
    event_loop.set_control_flow(ControlFlow::Poll);
    // ControlFlow::Poll keeps the event loop running continuously, ideal for real-time rendering.

    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        // run_app executes the app's main loop and handles errors returned from event handlers.
        eprintln!("Event loop error: {e}");
    }
}
