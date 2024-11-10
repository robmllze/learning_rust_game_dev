// A structure to represent a vertex with position and color, used in GPU buffers
#[repr(C)] // Ensures the struct has a C-compatible memory layout (important for working with GPU buffers)
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)] // Derives necessary traits for copying, cloning, and handling the struct in GPU code
pub struct Vertex {
    pub position: [f32; 4], // Position of the vertex in 4D space (x, y, z, w)
    pub color: [f32; 4],    // Color of the vertex (RGBA, each component between 0.0 and 1.0)
}

impl Vertex {
    // Function to define the vertex attributes, which describe the structure of the vertex in shaders
    pub fn vertex_attributes() -> Vec<wgpu::VertexAttribute> {
        // Define the attributes for the vertex: position (index 0) and color (index 1), both are 4-component floats
        wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4].to_vec() // Create a vector of vertex attributes
    }

    // Function to define the vertex buffer layout, which specifies how the data is laid out in memory
    pub fn description(attributes: &[wgpu::VertexAttribute]) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // The size of the Vertex struct in bytes, tells the GPU how much to advance in memory between vertices
            step_mode: wgpu::VertexStepMode::Vertex, // Specifies that each vertex has its own attributes (vertex-wise stepping)
            attributes, // The list of attributes that define the structure of the vertex (e.g., position and color)
        }
    }
}
