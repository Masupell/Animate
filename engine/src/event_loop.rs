use winit::{dpi::LogicalSize, event::*, event_loop::EventLoop, window::WindowBuilder};

use crate::{input::Input, renderer::Renderer, state::{Loader, LoadingContext, State}};

pub trait EngineEvent 
{
    fn setup(&mut self, loader: &mut dyn Loader);
    fn update(&mut self, input: &Input, dt: f64);
    fn render(&self, renderer: &mut Renderer);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn game_loop<T: EngineEvent + 'static>(mut game: Box<T>, title: &str, size: (i32, i32))
{
    cfg_if::cfg_if! 
    {
        if #[cfg(target_arch = "wasm32")] 
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        } else 
        {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title(title).with_inner_size(LogicalSize::new(size.0, size.1)).build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;

        use winit::platform::web::WindowExtWebSys;
        web_sys::window().and_then(|win| win.document()).and_then(|doc| 
        {
            let dst = doc.get_element_by_id("wasm-example")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok()?;
            Some(())
        }).expect("Couldn't append canvas to document body.");

        let _ = window.request_inner_size(PhysicalSize::new(450, 400));
    }

    let mut state = State::new(&window).await;
    let mut surface_configured = false;
    let size = window.inner_size();
    let mut input = Input::new((size.width as f64, size.height as f64));

    let mut loader = LoadingContext::new(&mut state.renderer, &mut state.device, &mut state.queue);
    game.setup(&mut loader);

    let mut last_frame_time = std::time::Instant::now();

    event_loop.run(move | event, control_flow |
    {
        match event
        {
            Event::WindowEvent 
            {
                ref event,
                window_id,
            }
            if window_id == state.window().id() => 
            {
                input.update_inputs(&event);
                if !state.input(event){
                match event
                {
                    WindowEvent::CloseRequested => control_flow.exit(),
                    WindowEvent::Resized(physical_size) => 
                    {
                        log::info!("physical_size: {physical_size:?}");
                        surface_configured = true;
                        state.resize(*physical_size);
                        input.update_screen((physical_size.width as f64, physical_size.height as f64));
                    }
                    WindowEvent::RedrawRequested => 
                    {
                        // state.window().request_redraw();

                        if !surface_configured 
                        {
                            return;
                        }


                        ///////////////////////////////////////////////////
                        state.update(); // Temporary
                        // state.input(&event); //Temporary
                        ///////////////////////////////////////////////////
                        match state.render(|renderer|
                        {
                            game.render(renderer);
                        })
                        {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => 
                            {
                                log::error!("OutOfMemory");
                                control_flow.exit();
                            }

                            Err(wgpu::SurfaceError::Timeout) => 
                            {
                                log::warn!("Surface timeout")
                            }
                        }
                    }
                    _ => {}
                }}
            }
            Event::AboutToWait =>
            {
                let now = std::time::Instant::now();
                let dt = (now - last_frame_time).as_secs_f64();
                
                game.update(&input, dt);
                state.window().request_redraw();
                input.prev_update();

                last_frame_time = std::time::Instant::now();
            }
            _ => {}
        }
    }).unwrap();
}