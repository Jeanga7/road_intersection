extern crate sdl2;

mod road;
mod simulation;
mod traffic_light;
mod vehicle;
mod graphics;

pub use road::*;
pub use simulation::*;
pub use graphics::*;
pub use traffic_light::*;
pub use vehicle::*;

fn main() {
    let mut simulation = Simulation::new();
    let mut graphics = Graphics::new("ROAD INTERSECTION", 800, 800);
    let mut intersection = Intersection::new();

    let mut event_pump = graphics.context.event_pump().unwrap();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                _ => {}
            }
        }

        simulation.update();
        intersection.update();

        graphics.clear();
        simulation.render(&mut graphics.canvas);
        intersection.draw(&mut graphics.canvas);

        graphics.present();
    }
}
