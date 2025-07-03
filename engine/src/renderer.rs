use wgpu::util::DeviceExt;

use crate::utility::{DrawCommand, InstanceData, Vertex};



pub const QUAD_VERTICES: &[Vertex] =
&[
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 0.0] }
];

pub const QUAD_INDICES: &[u16] =
&[
    0, 1, 2,
    2, 3, 0
];



pub struct Renderer
{
    pub pipeline: wgpu::RenderPipeline,
    pub draw_commands: Vec<DrawCommand>,
    instance_buf: Option<wgpu::Buffer>,
    vertex_buf: wgpu::Buffer,
    index_buf:  wgpu::Buffer,
    index_count: u32,
}

impl Renderer
{
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor
        {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor
        {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: 
            &[
                
            ],
            push_constant_ranges: &[]
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor
        {
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState 
            {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc(), InstanceData::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState
            {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState
                {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            }),
            primitive: wgpu::PrimitiveState
            {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill, //::Line only work with required_features: wgpu::Features::POLYGON_MODE_LINE in request device
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState
            {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None,
            cache: None
        });


        let vertices = 
        [
            Vertex::new([-0.0868241, 0.49240386, 0.0], [0.4131759, 0.00759614]),
            Vertex::new([-0.49513406, 0.06958647, 0.0], [0.0048659444, 0.43041354]),
            Vertex::new([-0.21918549, -0.44939706, 0.0], [0.28081453, 0.949397]),
            Vertex::new([0.35966998, -0.3473291, 0.0], [0.85967, 0.84732914]),
            Vertex::new([0.44147372, 0.2347359, 0.0], [0.9414737, 0.2652641])
        ];

        let indices: &[u16] = 
        &[
            0, 1, 4,
            1, 2, 4,
            2, 3, 4,
        ];

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor 
        {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor 
        {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = QUAD_INDICES.len() as u32;

        Self 
        { 
            pipeline,
            draw_commands: Vec::new(),
            instance_buf: None,
            vertex_buf,
            index_buf,
            index_count
        }
    }

    pub fn begin_pass(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView)
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor 
        {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment 
            {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations 
                {
                    load: wgpu::LoadOp::Clear(wgpu::Color 
                    {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        if let Some(ref instance_buf) = self.instance_buf 
        {
            render_pass.set_vertex_buffer(1, instance_buf.slice(..));
            render_pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.index_count, 0, 0..self.draw_commands.len() as u32);
        }
        // render_pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        // render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }

    pub fn test_draw(&mut self, transform: [[f32; 4]; 4], color: [f32; 4])
    {
        self.draw_commands.push(DrawCommand { transform, color });
    }

    pub fn upload_instances(&mut self, device: &wgpu::Device)
    {
        let instances: Vec<InstanceData> = self.draw_commands.iter().map(|cmd| InstanceData
        {
            model: cmd.transform,
            color: cmd.color
        }).collect();

        self.instance_buf = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor
        {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX
        }));
    }
}