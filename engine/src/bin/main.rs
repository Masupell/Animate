use engine::*;

struct App {}

impl EngineEvent for App
{
    fn update(&mut self, _dt: f64) {}
    fn render(&self) {}
}

fn main()
{
    pollster::block_on(game_loop(Box::new(App{})))
}