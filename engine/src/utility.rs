#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2]
}

impl Vertex
{
    pub fn new(pos: [f32; 3], tex_pos: [f32; 2]) -> Self
    {
        Vertex
        {
            position: pos,
            tex_coords: tex_pos
        }
    }
    
    pub fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        wgpu::VertexBufferLayout
        {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes:
            &[
                wgpu::VertexAttribute
                {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute
                {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}



#[derive(Copy, Clone)]
pub struct DrawCommand
{
    pub mesh_id: usize,
    pub transform: [[f32; 4]; 4], // 4x4 model matrix
    pub color: [f32; 4],          // RGBA color
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData 
{
    pub model: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl InstanceData
{
    pub fn desc() -> wgpu::VertexBufferLayout<'static> 
    {
        wgpu::VertexBufferLayout 
        {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: 
            &[
                wgpu::VertexAttribute 
                {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute 
                {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute 
                {
                    offset: 2 * std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute 
                {
                    offset: 3 * std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Color vec4
                wgpu::VertexAttribute 
                {
                    offset: 4 * std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}


pub struct Mesh
{
    pub vertex_buf: wgpu::Buffer,
    pub index_buf: wgpu::Buffer,
    pub index_count: u32
}

pub enum MeshID
{
    QUAD = 0
}