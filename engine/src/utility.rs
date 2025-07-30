use std::sync::Arc;

use wgpu::ShaderModule;

use crate::{shader::Shader, texture::Texture};

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
pub enum DrawType
{
    Color([f32; 4]),
    Texture(u32)
}

#[derive(Copy, Clone)]
pub struct DrawCommand
{
    pub mesh_id: usize,
    pub transform: [[f32; 4]; 4], // 4x4 model matrix
    // pub color: [f32; 4],          // RGBA color
    pub kind: DrawType,
    pub z_index: u32
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceData 
{
    pub model: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub mode: u32, //0 = color, 1 = texture
    pub texture_id: u32,
    pub _padding: [u32; 2] //Padding to keep alignement
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
                wgpu::VertexAttribute //mode
                {
                    offset: 5 * std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32
                },
                wgpu::VertexAttribute //texture id
                {
                    offset: 5 * std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress + 4,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Uint32
                }
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



pub struct Material
{
    pub shader: Arc<Shader>,
    pub texture: Option<Arc<Texture>>
}

impl Material
{
    pub fn new(shader: Arc<Shader>, texture: Option<Arc<Texture>>) -> Self
    {
        Material 
        { 
            shader,
            texture
        }   
    }
}