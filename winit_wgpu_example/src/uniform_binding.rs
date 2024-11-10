// A structure to represent the uniform data that will be passed to shaders
#[repr(C)]
// Specifies that this struct should have the C ABI layout (important for interoperability with GPU buffers)
#[derive(Default, Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)] // Derives several traits for easy handling of the struct in GPU code
pub struct UniformBuffer {
    pub mvp: nalgebra_glm::Mat4, // MVP matrix (Model-View-Projection) to transform vertices in shaders
}

// A structure to manage the uniform buffer and its associated bind group in the GPU
pub struct UniformBinding {
    pub buffer: wgpu::Buffer, // The actual GPU buffer that stores the uniform data
    pub bind_group: wgpu::BindGroup, // The bind group that will be used to bind the buffer to shaders
    pub bind_group_layout: wgpu::BindGroupLayout, // The layout of the bind group that defines how resources are bound to shaders
}

impl UniformBinding {
    // Constructor to create a new UniformBinding, allocating buffer and bind group
    pub fn new(device: &wgpu::Device) -> Self {
        // Create a uniform buffer that can be used for writing data and binding to shaders
        let buffer = wgpu::util::DeviceExt::create_buffer_init(
            device, // Device to create the buffer on
            &wgpu::util::BufferInitDescriptor {
                // Descriptor defining how the buffer is initialized
                label: Some("Uniform Buffer"), // Label for debugging
                contents: bytemuck::cast_slice(&[UniformBuffer::default()]), // Initial contents (zeroed by default)
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, // Usage flags: this buffer will be used as a uniform and can be copied to
            },
        );

        // Create a bind group layout which defines how the uniform buffer will be accessed in shaders
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                // Entry for the uniform buffer in the bind group
                binding: 0, // Binding index (to match the index used in the shader)
                visibility: wgpu::ShaderStages::VERTEX, // This uniform will only be used in the vertex shader
                ty: wgpu::BindingType::Buffer {
                    // The type of binding (uniform buffer)
                    ty: wgpu::BufferBindingType::Uniform, // The buffer type is uniform
                    has_dynamic_offset: false, // This buffer does not use dynamic offsets
                    min_binding_size: None,    // No minimum size restriction
                },
                count: None, // Only one buffer is expected for this binding
            }],
            label: Some("uniform_bind_group_layout"), // Label for debugging
        });

        // Create the bind group that binds the buffer to the shader
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout, // Layout to define how the bind group is structured
            entries: &[wgpu::BindGroupEntry {
                // The actual entry in the bind group for the uniform buffer
                binding: 0, // Binding index (must match the shader)
                resource: buffer.as_entire_binding(), // Bind the entire buffer to this entry
            }],
            label: Some("uniform_bind_group"), // Label for debugging
        });

        // Return the created uniform binding, including the buffer, bind group, and bind group layout
        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    // Method to update the content of the uniform buffer
    pub fn update_buffer(
        &mut self, // Mutable reference to the uniform binding, since we are updating the buffer
        queue: &wgpu::Queue, // The queue to issue the write command to the GPU
        offset: wgpu::BufferAddress, // The offset within the buffer where the data should be written
        uniform_buffer: UniformBuffer, // The new uniform data to update the buffer with
    ) {
        // Write the updated uniform data to the buffer at the specified offset
        queue.write_buffer(
            &self.buffer,                            // The buffer to write to
            offset, // The offset within the buffer where the new data should be written
            bytemuck::cast_slice(&[uniform_buffer]), // Convert the UniformBuffer struct into a byte slice for writing
        );
    }
}
