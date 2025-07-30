use std::iter;
use winit::{event::*,event_loop::EventLoop,window::{Window, WindowBuilder}};

use crate::{renderer::Renderer, utility::MeshID};

pub struct State<'a> 
{
    surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: &'a Window,
    pub renderer: Renderer
}

impl<'a> State<'a> 
{
    pub async fn new(window: &'a Window) -> State<'a> 
    {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor 
        {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions 
        {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor 
        {
            label: None,
            required_features: wgpu::Features::POLYGON_MODE_LINE,  // empty()
            required_limits: if cfg!(target_arch = "wasm32") 
            {
                wgpu::Limits::downlevel_webgl2_defaults()
            } 
            else 
            {
                wgpu::Limits::default()
            },
            memory_hints: Default::default(),
        },None,).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration 
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let size = window.inner_size();
        let renderer = Renderer::new(&device, &queue, &config, (size.width as f32, size.height as f32));

        Self 
        {
            surface,
            device,
            queue,
            config,
            size,
            window,
            renderer
        }
    }

    pub fn window(&self) -> &Window 
    {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) 
    {
        if new_size.width > 0 && new_size.height > 0 
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.renderer.window_size = (new_size.width as f32, new_size.height as f32);
        }
    }

    #[allow(unused)]
    pub fn input(&mut self, event: &WindowEvent) -> bool { false }

    #[allow(unused)]
    pub fn update(&mut self) {}

    pub fn render<T>(&mut self, draw: T) -> Result<(), wgpu::SurfaceError> where T: FnOnce(&mut Renderer)
    {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor 
        {
            label: Some("Render Encoder"),
        });


        // self.renderer.draw(0, [[1.75, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
        // self.renderer.draw(0, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.75, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
        // self.renderer.draw(0, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [1.0, 0.0, 0.0, 1.0]);
        draw(&mut self.renderer);
        self.renderer.upload_instances(&self.device);
        {
            self.renderer.begin_pass(&mut encoder, &view);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        self.renderer.draw_commands.clear();

        Ok(())
    }

    pub fn load_texture(&mut self, path: &str) -> usize // Returns ID
    {
        self.renderer.load_texture(&self.device, &self.queue, path)
    }
}


pub trait Loader
{
    fn load_texture(&mut self, path: &str) -> usize;
}

pub struct LoadingContext<'a> 
{
    renderer: &'a mut Renderer,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> LoadingContext<'a>
{
    pub fn new(renderer: &'a mut Renderer, device: &'a wgpu::Device, queue: &'a wgpu::Queue,) -> Self
    {
        Self { renderer, device, queue }
    }
}

impl<'a> Loader for LoadingContext<'a> 
{
    fn load_texture(&mut self, path: &str) -> usize 
    {
        self.renderer.load_texture(self.device, self.queue, path)
    }
}