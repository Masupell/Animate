use std::f32::consts::PI;

use engine::*;
use winit::{event::MouseButton, keyboard::KeyCode};

struct App 
{
    rotation: f32,
    x: f32,
    y: f32
}

impl EngineEvent for App
{
    fn update(&mut self, input: &Input, dt: f64) 
    {
        if input.is_mouse_pressed(MouseButton::Left)
        {
            println!("Position: {:?}", input.mouse_position());
        }

        if input.is_key_pressed(KeyCode::Tab)
        {
            println!("Tab pressed");
        }

        // if input

        self.rotation += 0.1745329*dt as f32;
        if self.rotation >= 2.0*PI
        {
            self.rotation = 0.0;
        }

        self.x = (input.mouse_position().0 as f32 / 512.0) * 2.0 - 1.0; //Temporary (just playing around), later better with projection matrix
        self.y = -((input.mouse_position().1 as f32 / 512.0) * 2.0 - 1.0);
    }
    fn render(&self, renderer: &mut Renderer) //Column-major layout
    {
        renderer.draw(0, [[self.rotation.cos(), self.rotation.sin(), 0.0, 0.0], [-self.rotation.sin(), self.rotation.cos(), 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
        renderer.draw(0, [[(-self.rotation).cos(), (-self.rotation).sin(), 0.0, 0.0], [-(-self.rotation).sin(), (-self.rotation).cos(), 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [1.0, 0.0, 0.0, 1.0]);
        renderer.draw(0, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [self.x, self.y, 0.0, 1.0]], [0.0, 1.0, 0.0, 1.0]);
    }
}

impl App
{
    fn new() -> Self
    {
        Self { rotation: 0.0, x: 0.0, y: 0.0 }
    }
}

fn main()
{
    pollster::block_on(game_loop(Box::new(App::new()), "Animate", (512, 512)))
}