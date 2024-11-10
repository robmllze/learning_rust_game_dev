// Gpu struct: Encapsulates all the GPU-related components used for rendering with wgpu.
pub struct Gpu<'window> {
    pub surface: wgpu::Surface<'window>,          // Surface for rendering. This connects to the window.
    pub device: wgpu::Device,                     // The device represents the GPU itself.
    pub queue: wgpu::Queue,                       // The queue to which rendering commands are submitted.
    pub surface_config: wgpu::SurfaceConfiguration, // Configuration for the surface (screen size, format, etc.).
    pub surface_format: wgpu::TextureFormat,      // The format in which the surface (screen) will be rendered.
}

impl<'window> Gpu<'window> {
    // Method to calculate the aspect ratio of the surface.
    pub fn aspect_ratio(&self) -> f32 {
        // Dividing the width by height. Prevent dividing by 0 by using `.max(1)` on height.
        self.surface_config.width as f32 / self.surface_config.height.max(1) as f32
    }

    // Method to resize the surface (e.g., when the window is resized).
    pub fn resize(&mut self, width: u32, height: u32) {
        // Update the surface configuration with new width and height.
        self.surface_config.width = width;
        self.surface_config.height = height;
        // Reconfigure the surface to apply the new size.
        self.surface.configure(&self.device, &self.surface_config);
    }

    // Creates a depth texture used for depth buffering (useful for 3D rendering).
    pub fn create_depth_texture(&self, width: u32, height: u32) -> wgpu::TextureView {
        // Create a texture with depth format (Depth32Float). This is used for depth testing.
        let texture = self.device.create_texture(
            &(wgpu::TextureDescriptor {
                label: Some("Depth Texture"), // Label for debugging purposes.
                size: wgpu::Extent3d {
                    width, // Width of the texture (same as window size).
                    height, // Height of the texture (same as window size).
                    depth_or_array_layers: 1, // We are creating a 2D texture, so depth is 1.
                },
                mip_level_count: 1, // No mipmaps, we are not using them here.
                sample_count: 1, // No anti-aliasing.
                dimension: wgpu::TextureDimension::D2, // 2D texture.
                format: wgpu::TextureFormat::Depth32Float, // Format for depth.
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, // Specify the texture usage.
                view_formats: &[], // No alternative formats for the texture.
            }),
        );

        // Create and return a view to the depth texture. This allows us to bind the texture to a pipeline for use.
        texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(wgpu::TextureFormat::Depth32Float), // Depth texture format.
            dimension: Some(wgpu::TextureViewDimension::D2), // 2D texture view.
            aspect: wgpu::TextureAspect::All, // View all aspects of the texture (i.e., the whole texture).
            base_mip_level: 0, // Start from the first mip level (no mipmaps).
            base_array_layer: 0, // Start from the first array layer.
            array_layer_count: None, // No array layers.
            mip_level_count: None, // No mipmap levels.
        })
    }

    // Asynchronous method to create a new Gpu instance (this is where most of the GPU setup happens).
    pub async fn new_async(
        window: impl Into<wgpu::SurfaceTarget<'window>>, // Input window (surface target for rendering).
        width: u32,   // Width of the window (and surface).
        height: u32,  // Height of the window (and surface).
    ) -> Self {
        // Step 1: Initialize wgpu instance (this is the entry point for using the GPU).
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all),
            // `backends_from_env()` checks environment variables for GPU backend preferences (e.g., Vulkan, DX12, Metal).
            ..Default::default() // Default settings for other instance properties.
        });

        // Step 2: Create the surface from the provided window.
        let surface = instance.create_surface(window).unwrap();

        // Step 3: Request a suitable adapter for the GPU (device that can render to our surface).
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), // Choose a GPU based on power preferences.
                compatible_surface: Some(&surface), // The adapter must be compatible with the surface.
                force_fallback_adapter: false, // Don't force fallback if no suitable adapter is found.
            })
            .await
            .expect("Failed to request adapter!");

        // Step 4: Request a device and queue from the adapter.
        let (device, queue) = {
            log::info!("WGPU Adapter Features: {:#?}", adapter.features());
            // Request the device (actual GPU) with specific configurations.
            adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("WGPU Device"), // Label for the device (useful for debugging).
                        required_features: wgpu::Features::default(), // No special GPU features required.
                        required_limits: wgpu::Limits {
                            max_texture_dimension_2d: 4096, // Support larger textures.
                            ..wgpu::Limits::downlevel_defaults() // Defaults for lower-tier devices.
                        },
                        memory_hints: wgpu::MemoryHints::default(), // Memory management hints.
                    },
                    None, // No special extensions.
                )
                .await
                .expect("Failed to request a device!")
        };

        // Step 5: Get the surface capabilities (e.g., formats, present modes).
        let surface_capabilities = surface.get_capabilities(&adapter);

        // Step 6: Choose a surface format that is compatible with the GPU and not sRGB (for non-color-managed rendering).
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| !f.is_srgb()) // We avoid sRGB formats for certain rendering needs.
            .unwrap_or(surface_capabilities.formats[0]); // Fallback to the first format if no non-sRGB format is available.

        // Step 7: Set the surface configuration (defines how the surface behaves).
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // The surface will be used for rendering.
            format: surface_format, // The surface format (selected above).
            width,  // Set the surface width.
            height, // Set the surface height.
            present_mode: surface_capabilities.present_modes[0], // The presentation mode (how frames are presented to the screen).
            alpha_mode: surface_capabilities.alpha_modes[0], // How alpha blending is handled (transparent or opaque).
            view_formats: vec![], // No additional view formats.
            desired_maximum_frame_latency: 2, // Max frames to keep in-flight for low latency.
        };

        // Step 8: Configure the surface with the device and configuration.
        surface.configure(&device, &surface_config);

        // Return the new GPU struct instance with all components initialized.
        Self {
            surface,
            device,
            queue,
            surface_config,
            surface_format,
        }
    }
}