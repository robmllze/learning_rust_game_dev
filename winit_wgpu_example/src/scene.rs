// Import necessary modules for rendering, uniform binding, and vertex handling
use crate::renderer::Renderer;
use crate::uniform_binding::{UniformBinding, UniformBuffer};
use crate::vertex::Vertex;

// Define the `Scene` struct which holds the model matrix, buffers, uniform binding, and render pipeline
pub struct Scene {
    pub model: nalgebra_glm::Mat4,   // Transformation matrix for the scene
    pub vertex_buffer: wgpu::Buffer, // Vertex buffer for storing vertex data
    pub index_buffer: wgpu::Buffer,  // Index buffer for storing indices to draw primitives
    pub uniform: UniformBinding,     // Uniform binding to pass data to shaders
    pub pipeline: wgpu::RenderPipeline, // The pipeline for rendering the scene
}

impl Scene {
    // Constructor function to create a new `Scene` instance
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        // Create the vertex buffer by initializing it with vertex data (using the `VERTICES` array)
        let vertex_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),              // Label for the buffer
                contents: bytemuck::cast_slice(&VERTICES), // Vertex data to be copied into the buffer
                usage: wgpu::BufferUsages::VERTEX,         // Usage: this is for storing vertex data
            },
        );

        // Create the index buffer by initializing it with index data (using the `INDICES` array)
        let index_buffer = wgpu::util::DeviceExt::create_buffer_init(
            device,
            &wgpu::util::BufferInitDescriptor {
                label: Some("index Buffer"),              // Label for the buffer
                contents: bytemuck::cast_slice(&INDICES), // Index data for rendering primitives
                usage: wgpu::BufferUsages::INDEX,         // Usage: this is for storing indices
            },
        );

        // Create a new uniform binding (for sending data to shaders)
        let uniform = UniformBinding::new(device);

        // Create the render pipeline (which handles shader execution and drawing)
        let pipeline = Self::create_pipeline(device, surface_format, &uniform);

        // Return a new `Scene` instance with default values and the created buffers and pipeline
        Self {
            model: nalgebra_glm::Mat4::identity(), // Initialize the model matrix as an identity matrix
            uniform,                               // The created uniform binding
            pipeline,                              // The created render pipeline
            vertex_buffer,                         // The created vertex buffer
            index_buffer,                          // The created index buffer
        }
    }

    // Render method to draw the scene using the pipeline and bind the required buffers
    pub fn render<'rpass>(&'rpass self, renderpass: &mut wgpu::RenderPass<'rpass>) {
        // Set the render pipeline to be used for this render pass
        renderpass.set_pipeline(&self.pipeline);

        // Bind the uniform buffer to the pipeline at binding group 0
        renderpass.set_bind_group(0, &self.uniform.bind_group, &[]);

        // Set the vertex buffer for the pipeline
        renderpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // Set the index buffer for the pipeline
        renderpass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        // Draw the indexed geometry (using the index buffer) for the defined primitive
        renderpass.draw_indexed(0..(INDICES.len() as _), 0, 0..1);
    }

    // Update method to modify the scene, including the transformation matrix and uniform data
    pub fn update(&mut self, queue: &wgpu::Queue, aspect_ratio: f32, delta_time: f32) {
        // Create the projection matrix based on the aspect ratio and field of view
        let projection =
            nalgebra_glm::perspective_lh_zo(aspect_ratio, 80_f32.to_radians(), 0.1, 1000.0);

        // Create the view matrix, which is a camera looking at the origin from a certain position
        let view = nalgebra_glm::look_at_lh(
            &nalgebra_glm::vec3(0.0, 0.0, 3.0), // Camera position (3 units away on the Z-axis)
            &nalgebra_glm::vec3(0.0, 0.0, 0.0), // The point the camera is looking at (the origin)
            &nalgebra_glm::Vec3::y(),           // Up direction is along the Y-axis
        );

        // Rotate the model matrix by a small amount over time (to animate it)
        self.model = nalgebra_glm::rotate(
            &self.model,
            30_f32.to_radians() * delta_time, // Rotate by 30 degrees per second, scaled by delta_time
            &nalgebra_glm::Vec3::y(),         // Rotate around the Y-axis
        );

        // Update the uniform buffer with the new model-view-projection (MVP) matrix
        self.uniform.update_buffer(
            queue,
            0, // Binding slot 0
            UniformBuffer {
                mvp: projection * view * self.model, // Compute the final MVP matrix
            },
        );
    }

    // Helper function to create the render pipeline with shaders and configuration
    fn create_pipeline(
        device: &wgpu::Device,               // The wgpu device
        surface_format: wgpu::TextureFormat, // The surface format (pixel format) for the render target
        uniform: &UniformBinding,            // The uniform binding for passing data to shaders
    ) -> wgpu::RenderPipeline {
        // Create the shader module from the WGSL shader source code
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(SHADER_SOURCE)), // The shader source code
        });

        // Create the pipeline layout which defines the uniform bind groups for the pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&uniform.bind_group_layout], // Set the uniform bind group layout
            push_constant_ranges: &[],                         // No push constants are used here
        });

        // Create the actual render pipeline
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout), // Use the created pipeline layout
            vertex: wgpu::VertexState {
                module: &shader_module,     // The vertex shader module
                entry_point: "vertex_main", // Entry point for the vertex shader
                buffers: &[Vertex::description(&Vertex::vertex_attributes())], // Define vertex buffer layout
                compilation_options: Default::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip, // Use triangle strip primitive topology
                strip_index_format: Some(wgpu::IndexFormat::Uint32), // 32-bit unsigned integers for indices
                front_face: wgpu::FrontFace::Cw,                     // Clockwise front face
                cull_mode: None,                                     // No culling
                polygon_mode: wgpu::PolygonMode::Fill,               // Solid fill polygons
                conservative: false, // Do not enable conservative rasterization
                unclipped_depth: false, // Depth clipping enabled
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Renderer::DEPTH_FORMAT, // Use the depth format from the renderer
                depth_write_enabled: true,      // Enable depth writing
                depth_compare: wgpu::CompareFunction::Less, // Compare depth for writing
                stencil: wgpu::StencilState::default(), // Use default stencil state
                bias: wgpu::DepthBiasState::default(), // Default depth bias
            }),
            multisample: wgpu::MultisampleState {
                count: 1,                         // Single sample per pixel
                mask: !0,                         // No mask
                alpha_to_coverage_enabled: false, // Disable alpha to coverage
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,       // The fragment shader module
                entry_point: "fragment_main", // Entry point for the fragment shader
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format, // Use the surface format for the output color
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // Use alpha blending
                    write_mask: wgpu::ColorWrites::ALL, // Write to all color channels
                })],
                compilation_options: Default::default(),
            }),
            multiview: None, // No multiview (for multiple views)
            cache: None,     // No caching
        })
    }
}

// Define the vertex data for a triangle (3 vertices with position and color)
const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [1.0, -1.0, 0.0, 1.0], // Position of vertex 1
        color: [1.0, 0.0, 0.0, 1.0],     // Red color for vertex 1
    },
    Vertex {
        position: [-1.0, -1.0, 0.0, 1.0], // Position of vertex 2
        color: [0.0, 1.0, 0.0, 1.0],      // Green color for vertex 2
    },
    Vertex {
        position: [0.0, 1.0, 0.0, 1.0], // Position of vertex 3
        color: [0.0, 0.0, 1.0, 1.0],    // Blue color for vertex 3
    },
];

// Define the indices for the vertices, which describe how to connect the vertices to form a triangle
const INDICES: [u32; 3] = [
    0, // First vertex (index 0)
    1, // Second vertex (index 1)
    2, // Third vertex (index 2)
];

// Shader source code in WGSL (WebGPU Shading Language) for the vertex and fragment shaders
const SHADER_SOURCE: &str = "
struct Uniform {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> ubo: Uniform;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
};
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex_main(vert: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = vert.color;
    out.position = ubo.mvp * vert.position;
    return out;
};

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color);
}
";
