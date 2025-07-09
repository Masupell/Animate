use wgpu::util::DeviceExt;

use crate::{texture, utility::{DrawCommand, DrawType, InstanceData, Mesh, Vertex}};



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
    meshes: Vec<Mesh>, // Simple for now, later gonna change it, so it does not load all meshes ni the beginning, but only creates a mesh the first time it is requested
    pub window_size: (f32, f32),
    pub virtual_size: (f32, f32),
    diffuse_bind_group: wgpu::BindGroup
}

impl Renderer
{
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration, window_size: (f32, f32)) -> Self
    {
        //TextureTest
        let diffuse_bytes = include_bytes!("image/owl.jpg");
        let diffuse_texture = texture::Texture::from_bytes(device, queue, diffuse_bytes, "image").unwrap();
        let (texture_bindgroup_layout, diffuse_bind_group) = diffuse_texture.bind_group(&device);




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
                &texture_bindgroup_layout
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

        let quad_mesh = Mesh
        {
            vertex_buf,
            index_buf,
            index_count
        };

        let meshes = vec![quad_mesh];

        Self 
        { 
            pipeline,
            draw_commands: Vec::new(),
            instance_buf: None,
            meshes,
            window_size,
            virtual_size: window_size,
            diffuse_bind_group
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
        // render_pass.set_vertex_buffer(0, self.vertex_buf.slice(..));

        // if let Some(ref instance_buf) = self.instance_buf
        // {
        //     let mesh = &self.meshes[]

        //     render_pass.set_vertex_buffer(1, instance_buf.slice(..));
        //     render_pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        //     render_pass.draw_indexed(0..self.index_count, 0, 0..self.draw_commands.len() as u32);
        // }

        if let Some(ref instance_buf) = self.instance_buf
        {
            render_pass.set_vertex_buffer(1, instance_buf.slice(..));
            for (instance_id, cmd) in self.draw_commands.iter().enumerate()
            {
                let mesh = &self.meshes[cmd.mesh_id];

                render_pass.set_vertex_buffer(0, mesh.vertex_buf.slice(..));
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                render_pass.set_index_buffer(mesh.index_buf.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..mesh.index_count, 0, instance_id as u32..instance_id as u32 + 1);
            }
        }
    }

    pub fn draw(&mut self, mesh_id: usize, transform: [[f32; 4]; 4], color: [f32; 4])
    {
        self.draw_commands.push(DrawCommand { mesh_id, transform, kind: DrawType::Color(color) });
    }

    pub fn draw_texture(&mut self, mesh_id: usize, transform: [[f32; 4]; 4], texture_id: u32)
    {
        self.draw_commands.push(DrawCommand { mesh_id, transform, kind: DrawType::Texture(texture_id) });
    }

    pub fn upload_instances(&mut self, device: &wgpu::Device)
    {
        if self.draw_commands.is_empty()
        {
            self.instance_buf = None;
            return;
        }
        
        // let instances: Vec<InstanceData> = self.draw_commands.iter().map(|cmd| InstanceData
        // {
        //     model: cmd.transform,
        //     color: cmd.color
        // }).collect();

        let instances: Vec<InstanceData> = self.draw_commands.iter().map(|cmd|
        {
            match cmd.kind
            {
                DrawType::Color(color) => 
                InstanceData
                {
                    model: cmd.transform,
                    color: color,
                    mode: 0,
                    texture_id: 0,
                    _padding: [0; 2]
                },
                DrawType::Texture(id) => 
                InstanceData
                {
                    model: cmd.transform,
                    color: [0.0, 0.0, 0.0, 1.0], // Ignored here
                    mode: 1,
                    texture_id: id,
                    _padding: [0; 2]
                }
            }
        }).collect();

        self.instance_buf = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor
        {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX
        }));
    }

    // pos in pixels, size as in 1.0 is default scale, rotation in radians (all for 2D, would work for 3D, but this is 2D)
    pub fn to_matrix(&self, pos: (f32, f32), size: (f32, f32), rotation: f32) -> [[f32; 4]; 4]
    {
        let aspect = self.window_size.0/self.window_size.1;
        let scale = 1./aspect;

        let cos = rotation.cos();
        let sin = rotation.sin();

        [
            [scale*cos*size.0, sin*size.0, 0.0, 0.0], 
            [scale*-sin*size.1, cos*size.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0], 
            [(pos.0/self.window_size.0)*2.0-1.0, -((pos.1/self.window_size.1)*2.0-1.0), 0.0, 1.0]
        ]
    }

    // Size in pixels now too
    // Always stays the same size, even if screen gets resized (so always 100px big for example), so not relative says but static
    pub fn pixel_matrix(&self, pos: (f32, f32), size: (f32, f32), rotation: f32) -> [[f32; 4]; 4]
    {
        let aspect = self.window_size.0/self.window_size.1;
        let scale = 1./aspect;

        let cos = rotation.cos();
        let sin = rotation.sin();

        let pixel_size = ((size.0/self.window_size.1)*2.0, (size.1/self.window_size.1)*2.0);

        [
            [scale*cos*pixel_size.0, sin*pixel_size.0, 0.0, 0.0], 
            [scale*-sin*pixel_size.1, cos*pixel_size.1, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0], 
            [(pos.0/self.window_size.0)*2.0-1.0, -((pos.1/self.window_size.1)*2.0-1.0), 0.0, 1.0]
        ]
    }

    // Still draws with pixels, but this time everything gets drawn like it looks with the original screen-size, so resized looks the same (in relation to each other)
    // If using this, when trying to use the windowsize, use virtual_size instead of window_size
    // Because everything here is in relation to the original "virtual" size, not the actual window size
    pub fn matrix(&self, pos: (f32, f32), size: (f32, f32), rotation: f32) -> [[f32; 4]; 4]
    {
        let aspect = self.window_size.0/self.window_size.1;
        let scale = 1./aspect;

        let cos = rotation.cos();
        let sin = rotation.sin();

        let scale_x = (size.0/self.virtual_size.1)*2.0;
        let scale_y = (size.1/self.virtual_size.1)*2.0;

        [
            [scale*cos*scale_x, sin*scale_x, 0.0, 0.0], 
            [scale*-sin*scale_y, cos*scale_y, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0], 
            [(pos.0/self.virtual_size.0)*2.0-1.0, -((pos.1/self.virtual_size.1)*2.0-1.0), 0.0, 1.0]
        ]
    }
}