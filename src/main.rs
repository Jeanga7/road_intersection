extern crate sdl2;
extern crate rand;

mod graphics;
mod road;
mod simulation;
mod traffic_light;
mod vehicle;

pub use graphics::*;
pub use road::*;
pub use simulation::*;
pub use traffic_light::*;
pub use vehicle::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rand::Rng;

fn main() -> Result<(), String> {
    let mut simulation = Simulation::new();
    let mut graphics = Graphics::new("ROAD INTERSECTION", 800, 800);
    let mut event_pump = graphics.context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Up => simulation.spawn_vehicle(Direction::North),
                        Keycode::Down => simulation.spawn_vehicle(Direction::South),
                        Keycode::Left => simulation.spawn_vehicle(Direction::West),
                        Keycode::Right => simulation.spawn_vehicle(Direction::East),
                        Keycode::R => {
                            let mut rng = rand::thread_rng();
                            let direction = match rng.gen_range(0..4) {
                                0 => Direction::North,
                                1 => Direction::South,
                                2 => Direction::East,
                                _ => Direction::West,
                            };
                            simulation.spawn_vehicle(direction);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        simulation.update();
        
        graphics.clear();
        simulation.render(&mut graphics.canvas);
        graphics.present();

        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}