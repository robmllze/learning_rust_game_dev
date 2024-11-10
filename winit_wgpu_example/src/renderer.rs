use crate::gpu::Gpu; // Importing the Gpu struct from the gpu module.
use crate::scene::Scene;
pub use std::time::Duration; // Importing the Duration struct for working with time intervals. // Importing the Scene struct from the scene module.

/// The `Renderer` struct holds the GPU, depth texture view, and scene information for rendering.
pub struct Renderer<'window> {
    gpu: Gpu<'window>, // The GPU struct which handles GPU-related tasks.
    depth_texture_view: wgpu::TextureView, // The texture view for the depth buffer.
    scene: Scene,      // The scene to be rendered.
}

impl<'window> Renderer<'window> {
    // Defining a constant for the depth texture format.
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Asynchronous function to create a new renderer.
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'window>>, // The window for rendering.
        width: u32,                                      // Width of the window.
        height: u32,                                     // Height of the window.
    ) -> Self {
        // Initialize the GPU by creating an asynchronous GPU object.
        let gpu = Gpu::new_async(window, width, height).await;

        // Create a depth texture view for the renderer.
        let depth_texture_view = gpu.create_depth_texture(width, height);

        // Initialize the scene to be rendered.
        let scene = Scene::new(&gpu.device, gpu.surface_format);

        // Return a new `Renderer` object.
        Self {
            gpu,                // The GPU object.
            depth_texture_view, // The depth texture view.
            scene,              // The scene object.
        }
    }

    /// Function to resize the renderer and the associated depth texture view.
    pub fn resize(&mut self, width: u32, height: u32) {
        // Resizes the GPU surface configuration.
        self.gpu.resize(width, height);

        // Create a new depth texture view for the resized window.
        self.depth_texture_view = self.gpu.create_depth_texture(width, height);
    }

    /// Function to render a frame, typically called once per frame.
    pub fn render_frame(
        &mut self,
        delta_time: Duration, // The time that has passed since the last frame.
    ) {
        // Convert delta_time into seconds as a floating-point number.
        let delta_time = delta_time.as_secs_f32();

        // Update the scene with the GPU queue, aspect ratio, and delta_time.
        self.scene
            .update(&self.gpu.queue, self.gpu.aspect_ratio(), delta_time);

        // Create a new command encoder for the render pass.
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"), // Label for debugging purposes.
            });

        // Get the current texture from the surface to render onto.
        let surface_texture = self
            .gpu
            .surface
            .get_current_texture()
            .expect("Failed to get surface texture!");

        // Create a texture view for the surface texture.
        let surface_texture_view =
            surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor {
                    label: wgpu::Label::default(),          // Label for debugging.
                    aspect: wgpu::TextureAspect::default(), // Default texture aspect (all).
                    format: Some(self.gpu.surface_format),  // Surface format (e.g., RGBA).
                    dimension: None,                        // Default dimension (2D).
                    base_mip_level: 0,                      // Mipmap level to start from.
                    mip_level_count: None,                  // Mipmap level count.
                    base_array_layer: 0,                    // Base array layer.
                    array_layer_count: None,                // Layer count.
                });

        // Insert a debug marker for the render pass (optional debugging).
        encoder.insert_debug_marker("Render scene");

        // Begin the render pass, which is responsible for rendering the scene.
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"), // Label for the render pass.
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view, // The texture view we are rendering to.
                    resolve_target: None,        // No resolve target (used in MSAA).
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.19, // Red value for background color.
                            g: 0.24, // Green value for background color.
                            b: 0.42, // Blue value for background color.
                            a: 1.0,  // Alpha value (opaque).
                        }),
                        store: wgpu::StoreOp::Store, // Store the result in the surface texture.
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view, // The depth texture view.
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0), // Clear the depth buffer.
                        store: wgpu::StoreOp::Store,    // Store the result in the depth buffer.
                    }),
                    stencil_ops: None, // No stencil buffer used.
                }),
                timestamp_writes: None,    // No timestamp writes.
                occlusion_query_set: None, // No occlusion queries.
            });

            // Render the scene into the render pass.
            self.scene.render(&mut render_pass);
        }

        // Submit the encoded commands to the GPU queue.
        self.gpu.queue.submit(std::iter::once(encoder.finish()));

        // Present the surface texture to the screen.
        surface_texture.present();
    }
}
