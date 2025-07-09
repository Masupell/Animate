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

        self.x = input.mouse_position().0 as f32;// (input.mouse_position().0 as f32 / 1280.0) * 2.0 - 1.0; //Temporary (just playing around), later better with projection matrix
        self.y = input.mouse_position().1 as f32;// ((input.mouse_position().1 as f32 / 720.0) * 2.0 - 1.0);
    }
    fn render(&self, renderer: &mut Renderer) //Column-major layout
    {  
        // renderer.draw(0, [[scale*self.rotation.cos(), self.rotation.sin(), 0.0, 0.0], [scale*-self.rotation.sin(), self.rotation.cos(), 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
        // renderer.draw(0, [[scale*(-self.rotation).cos(), (-self.rotation).sin(), 0.0, 0.0], [scale*-(-self.rotation).sin(), (-self.rotation).cos(), 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [1.0, 0.0, 0.0, 1.0]);
        // renderer.draw(0, [[scale*1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [self.x, self.y, 0.0, 1.0]], [0.0, 1.0, 0.0, 1.0]);
        renderer.draw(0,test((1280.0, 720.0), (1280.0/2.0, 720.0/2.0), (1.0, 1.0), self.rotation), [0.0, 0.0, 1.0, 1.0]);
        renderer.draw(0,test((1280.0, 720.0), (1280.0/2.0, 720.0/2.0), (1.0, 1.0), -self.rotation), [1.0, 0.0, 0.0, 1.0]);
        renderer.draw(0, test((1280.0, 720.0), (self.x, self.y), (1.0, 1.0), 0.0), [0.0, 1.0, 0.0, 1.0]);
    }
}

// pos in pixels, size as in 1.0 is default scale, rotation in radians (all for 2D, would work for 3D, but this is 2D)
fn test(screen_size: (f32, f32), pos: (f32, f32), size: (f32, f32), rotation: f32) -> [[f32; 4]; 4]
{
    let aspect = screen_size.0/screen_size.1;
    let scale = 1./aspect;

    let cos = rotation.cos();
    let sin = rotation.sin();

    [
        [scale*cos*size.0, sin*size.0, 0.0, 0.0], 
        [scale*-sin*size.1, cos*size.1, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0], 
        [(pos.0/screen_size.0)*2.0-1.0, -((pos.1/screen_size.1)*2.0-1.0), 0.0, 1.0]
    ]
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
    pollster::block_on(game_loop(Box::new(App::new()), "Animate", (1280, 720)))
}