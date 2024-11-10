// Imports.
use std::sync::Arc; // Arc is a thread-safe reference-counting pointer, used to share ownership of a value across threads.

use winit::{
    dpi::PhysicalSize, // Represents the physical size of a window in device pixels.
    window::Window, // Window represents a window created using winit. WindowId uniquely identifies it.
};

// State.

pub struct State<'a> {
    // State represents the rendering state of the app. It ties the app to GPU resources.
    instance: wgpu::Instance, // Instance is the entry point to WGPU, managing device connections and surfaces.
    surface: wgpu::Surface<'a>, // Surface is used to render into a window.
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> State<'a> {
        // Creates a new rendering state for a given window.
        let instance = wgpu::Instance::default(); // Initializes WGPU for default configurations.
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        // Surface links WGPU rendering with the window.

        Self { instance, surface }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        println!("State resize")
        // Placeholder for handling resizing logic.
    }

    pub fn draw(&self) {
        println!("State draw")
        // Placeholder for rendering logic.
    }
}
