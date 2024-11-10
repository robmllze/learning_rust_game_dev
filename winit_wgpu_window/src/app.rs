// Imports.
use std::sync::Arc; // Arc is a thread-safe reference-counting pointer, used to share ownership of a value across threads.

use winit::{
    application::ApplicationHandler, // ApplicationHandler is the trait implemented to handle app lifecycle events.
    event::*,
    event_loop::ActiveEventLoop, // EventLoop is the core of event handling in winit.
    window::{Window, WindowId}, // Window represents a window created using winit. WindowId uniquely identifies it.
};

use crate::state::State;

// App.

#[derive(Default)] // Gives the members default values, e.g. numbers are 0, strings are empty, etc.
pub struct App<'a> {
    window: Option<Arc<Window>>, // Optional shared window instance.
    state: Option<State<'a>>,    // Optional rendering state.
}

impl ApplicationHandler for App<'_> {
    // ApplicationHandler is the trait for handling application lifecycle events.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // "resumed" is triggered when the application returns from a suspended state.
        println!("App resumed");
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            );
            // Creates a new window using the event loop.
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            // pollster blocks async execution to initialize State synchronously.
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        // Handles events tied to the window, such as resizing or closing.
        if id != self.window.as_ref().unwrap().id() {
            return; // Ignores events from unrelated windows.
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit(); // Exits the application when the window is closed.
            }
            WindowEvent::Resized(physical_size) => {
                println!("Resize requested");
                self.state.as_mut().unwrap().resize(physical_size);
                // Delegates resizing logic to the State instance.
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                self.state.as_ref().unwrap().draw();
                // Delegates rendering to the State instance.
            }
            _ => {}
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        // Triggered when the application goes into a suspended state.
        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        // Triggered when the application is about to exit.
        println!("App exiting");
    }
}
