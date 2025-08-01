use std::f32::consts::PI;

use engine::*;
use winit::{event::MouseButton, keyboard::KeyCode};

struct App 
{
    rotation: f32,
    x: f32,
    y: f32,
    cheetah: usize,
    owl: usize,
    char: usize,
    // chars: Vec<usize>
}

impl EngineEvent for App
{
    fn setup(&mut self, loader: &mut dyn state::Loader) 
    {
        self.owl = loader.load_texture("engine/src/image/owl.jpg");
        self.cheetah = loader.load_texture("engine/src/image/cheetah.jpg");
        self.char = loader.load_char('?').unwrap();
        // let text = "HelloWorld!";
        // for c in text.chars()
        // {
        //     self.chars.push(loader.load_char(c).unwrap());
        // }
        let test = loader.load_text("Test:qle-|p!", 200.0);
    }
    
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

        self.x = input.mouse_position().0 as f32;
        self.y = input.mouse_position().1 as f32;
    }
    fn render(&self, renderer: &mut Renderer) //Column-major layout
    {  
        // renderer.draw(0, [[scale*self.rotation.cos(), self.rotation.sin(), 0.0, 0.0], [scale*-self.rotation.sin(), self.rotation.cos(), 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
        // renderer.draw(0, [[scale*(-self.rotation).cos(), (-self.rotation).sin(), 0.0, 0.0], [scale*-(-self.rotation).sin(), (-self.rotation).cos(), 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [1.0, 0.0, 0.0, 1.0]);
        // renderer.draw(0, [[scale*1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [self.x, self.y, 0.0, 1.0]], [0.0, 1.0, 0.0, 1.0]);
        renderer.draw_texture(0, renderer.matrix((renderer.virtual_size.0/2.0, renderer.virtual_size.1/2.0), (renderer.virtual_size.0, renderer.virtual_size.1), 0.0), self.cheetah, 0);
        renderer.draw(0, renderer.matrix((renderer.virtual_size.0/2.0, renderer.virtual_size.1/2.0), (renderer.virtual_size.1/2.0, renderer.virtual_size.1/2.0), self.rotation), [0.0, 0.0, 1.0, 0.5], 1);
        renderer.draw(0, renderer.matrix((renderer.virtual_size.0/2.0, renderer.virtual_size.1/2.0), (renderer.virtual_size.1/2.0, renderer.virtual_size.1/2.0), -self.rotation), [1.0, 0.0, 0.0, 0.5], 2);
        renderer.draw(0, renderer.matrix((self.x, self.y), (100.0, 100.0), 0.0), [0.0, 1.0, 0.0, 1.0], 3);
        renderer.draw_texture(0, renderer.texture_matrix((100.0, 100.0), (0.5, 0.5), 0.0, (1920.0, 1014.0)), self.owl, 4);
        renderer.draw_texture(0, renderer.texture_matrix((500.0, 500.0), (1.0, 1.0), 0.0, (24.0, 39.0)), self.char, 4);

        renderer.draw_texture(0, renderer.texture_matrix((600.0, 500.0), (1.0, 01.0), 0.0, (796.0, 124.0)), 3, 5);

        // for (index, count) in self.chars.iter().enumerate()
        // {
        //     renderer.draw_texture(0, renderer.matrix((200.0 + index as f32 *70.0, 200.0), (50.0, 50.0), 0.0), *count, 5);
        // }
    }
}

impl App
{
    fn new() -> Self
    {
        Self 
        { 
            rotation: 0.0, 
            x: 0.0, 
            y: 0.0,
            cheetah: 0,
            owl: 0,
            char: 0,
            // chars: Vec::new()
        }
    }
}

fn main()
{
    pollster::block_on(game_loop(Box::new(App::new()), "Animate", (1280, 720)))
}