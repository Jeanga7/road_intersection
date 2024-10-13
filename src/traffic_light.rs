extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::vehicle::Direction;

#[derive(Copy, Clone, PartialEq)]
pub struct TrafficLight {
    pub is_green: bool,
    pub direction: Direction,
    pub timer: u32,
}


impl TrafficLight {
    pub fn new(direction: Direction) -> Self {
        TrafficLight {
            is_green: false,
            direction,
            timer: 0,
        }
    }

    pub fn update(&mut self, cycle_duration: u32) {
        self.timer += 1;
        if self.timer >= cycle_duration {
            self.toggle();
            self.timer = 0;
        }
    }

    pub fn toggle(&mut self) {
        self.is_green = !self.is_green; 
    }
}

#[derive(Clone, PartialEq)]
pub struct Intersection {
    pub traffic_lights: Vec<TrafficLight>,
    pub cycle_duration: u32,
    pub current_phase: u32,
}


impl Intersection {
    pub fn new() -> Self {
        let lights = vec![
            TrafficLight::new(Direction::North),
            TrafficLight::new(Direction::South),
            TrafficLight::new(Direction::East),
            TrafficLight::new(Direction::West),
        ];

        Intersection {
            traffic_lights: lights,
            cycle_duration: 2000,
            current_phase: 0,
        }
    }

    pub fn update(&mut self) {
        self.current_phase += 1;
        if self.current_phase >= self.cycle_duration * 2 {
            self.current_phase = 0;
        }

        for light in &mut self.traffic_lights {
            light.update(self.cycle_duration);
        }

        let is_north_south_green = self.current_phase < self.cycle_duration;
        self.traffic_lights[0].is_green = !is_north_south_green; 
        self.traffic_lights[1].is_green = is_north_south_green; 
        self.traffic_lights[2].is_green = is_north_south_green;
        self.traffic_lights[3].is_green = !is_north_south_green;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for (i, light) in self.traffic_lights.iter().enumerate() {
            let (x, y) = match i {
                0 => (325, 325),
                1 => (325, 455),
                2 => (455, 325),
                _ => (455, 455),
            };

            let color = if light.is_green {
                Color::RGB(0, 255, 0) 
            } else {
                Color::RGB(255, 0, 0) 
            };

            self.draw_circle(canvas, x + 10, y + 10, 10, color);
        }
    }

    pub fn draw_circle(&self, canvas: &mut Canvas<Window>, center_x: i32, center_y: i32, radius: i32, color: Color) {
        canvas.set_draw_color(color);
        for y in -radius..=radius {
            let x_offset = (radius as f64 * (1.0 - (y * y) as f64 / (radius * radius) as f64).sqrt()).round() as i32;
            canvas.draw_line((center_x - x_offset, center_y + y), (center_x + x_offset, center_y + y)).unwrap();
        }
    }
}
