use engine::*;
use winit::{event::MouseButton, keyboard::KeyCode};

struct App {}

impl EngineEvent for App
{
    fn update(&mut self, input: &Input, _dt: f64) 
    {
        if input.is_mouse_pressed(MouseButton::Left)
        {
            println!("Position: {:?}", input.mouse_position());
        }

        if input.is_key_pressed(KeyCode::Tab)
        {
            println!("Tab pressed");
        }
    }
    fn render(&self, renderer: &mut Renderer) 
    {
        renderer.draw(0, [[1.75, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
    }
}

fn main()
{
    pollster::block_on(game_loop(Box::new(App{})))
}