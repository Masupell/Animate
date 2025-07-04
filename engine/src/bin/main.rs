use engine::*;

struct App {}

impl EngineEvent for App
{
    fn update(&mut self, _dt: f64) {}
    fn render(&self, renderer: &mut Renderer) 
    {
        renderer.draw(0, [[1.75, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], [0.0, 0.0, 1.0, 1.0]);
    }
}

fn main()
{
    pollster::block_on(game_loop(Box::new(App{})))
}