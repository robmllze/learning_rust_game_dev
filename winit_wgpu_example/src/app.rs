use std::sync::Arc; // Arc is a thread-safe reference-counted smart pointer, used for shared ownership across threads.
use winit::{
    application::ApplicationHandler, // Trait to handle application-level events in winit.
    dpi::PhysicalSize,               // Represents the size of the window in physical pixels.
    event::WindowEvent,              // Enum for window-related events (resize, close, etc.).
    window::Window,                  // The window object for rendering.
};

use crate::renderer::Renderer; // Import the Renderer struct from the renderer module.

pub use std::time::Instant; // Re-export `Instant` for easy access in other parts of the program.

#[derive(Default)] // Automatically implement the Default trait to create an App with default values.
pub struct App {
    window: Option<Arc<Window>>, // Optional reference to the window (wrapped in Arc for shared ownership).
    renderer: Option<Renderer<'static>>, // Optional reference to the renderer (renderer type is defined in a different module).
    last_render_time: Option<Instant>, // The time of the last frame rendered, used to calculate frame timings.
    last_size: (u32, u32),             // Stores the last window size (width, height).
}

impl ApplicationHandler for App {
    // The `resumed` method is called when the application is resumed.
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Create window attributes with default values.
        let mut attributes = Window::default_attributes();

        // Set window title to "Standalone Winit/Wgpu Example".
        attributes = attributes.with_title("Standalone Winit/Wgpu Example");

        // Try to create a window and handle errors.
        if let Ok(window) = event_loop.create_window(attributes) {
            let first_window_handle = self.window.is_none(); // Check if this is the first window creation.
            let window_handle = Arc::new(window); // Wrap the window in an Arc to share ownership across threads.
            self.window = Some(window_handle.clone()); // Store the window handle.

            // If it's the first window, initialize the renderer and store the last size.
            if first_window_handle {
                let inner_size = window_handle.inner_size();
                self.last_size = (inner_size.width, inner_size.height); // Store initial window size.

                // Get the width and height of the window.
                let (width, height) = (
                    window_handle.inner_size().width,
                    window_handle.inner_size().height,
                );

                env_logger::init(); // Initialize the logger for logging purposes.
                                    // Create the renderer asynchronously using the window handle and size.
                let renderer = pollster::block_on(async move {
                    Renderer::new(window_handle.clone(), width, height).await
                });
                self.renderer = Some(renderer); // Store the renderer instance.
                self.last_render_time = Some(Instant::now()); // Initialize the render time.
            }
        }
    }

    // The `window_event` method handles events related to the window.
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop, // The event loop to drive window events.
        _window_id: winit::window::WindowId,             // Window ID, not used in this example.
        event: winit::event::WindowEvent, // The specific window event (resize, keyboard input, etc.).
    ) {
        // Ensure that the renderer, window, and last render time are all initialized before proceeding.
        let (Some(renderer), Some(window), Some(last_render_time)) = (
            self.renderer.as_mut(),
            self.window.as_ref(),
            self.last_render_time.as_mut(),
        ) else {
            return; // If any of these are not initialized, return early.
        };

        match event {
            // If a keyboard key is pressed.
            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: winit::keyboard::PhysicalKey::Code(key_code), // Get the keycode.
                        ..
                    },
                ..
            } => {
                // Exit the application if the escape key is pressed.
                if matches!(key_code, winit::keyboard::KeyCode::Escape) {
                    event_loop.exit(); // Exit the event loop and close the application.
                }
            }
            // If the window is resized.
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                // Ensure the new width and height are at least 1 pixel.
                let (width, height) = ((width).max(1), (height).max(1));
                log::info!("Resizing renderer surface to: ({width}, {height})"); // Log the resizing action.
                renderer.resize(width, height); // Resize the renderer's surface.
                self.last_size = (width, height); // Update the last known window size.
            }
            // If the window close button is pressed.
            WindowEvent::CloseRequested => {
                log::info!("Close requested. Exiting..."); // Log the close request.
                event_loop.exit(); // Exit the event loop and close the application.
            }
            // If the window requests a redraw (i.e., the screen needs to be rendered again).
            WindowEvent::RedrawRequested => {
                let now = Instant::now(); // Get the current time.
                let delta_time = now - *last_render_time; // Calculate the time difference since the last render.
                *last_render_time = now; // Update the last render time.
                renderer.render_frame(delta_time); // Render a new frame with the delta time.
            }
            _ => (), // For any other window events, do nothing.
        }

        // Request the window to be redrawn after handling the event.
        window.request_redraw();
    }
}
