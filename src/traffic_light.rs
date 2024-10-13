extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

#[derive(Copy, Clone)]
pub struct TrafficLight {
    pub is_green: bool,
    pub change_color: bool, 
    pub counter: u32,
}

impl TrafficLight {
    pub fn new() -> Self {
        TrafficLight {
            is_green: false,
            change_color: false, 
            counter: 0,
        }
    }

    pub fn update(&mut self) {
        if self.change_color {
            self.toggle(); 
            self.change_color = false;
        }
        
        self.counter += 1; 
    }

    pub fn toggle(&mut self) {
        self.is_green = !self.is_green; 
    }
}

pub struct Intersection {
    pub traffic_lights: Vec<TrafficLight>, 
    pub current_light: usize,
    pub change_light: bool,
    pub counter: u32,
}

impl Intersection {
    pub fn new() -> Self {
        let lights = vec![TrafficLight::new(); 4];

        Intersection {
            traffic_lights: lights,
            current_light: 0,
            change_light: false, 
            counter: 0,
        }
    }

    pub fn update(&mut self) {
        if let Some(light) = self.traffic_lights.get_mut(self.current_light) {
            light.update();
            if light.counter >= 1000 { 
                light.change_color = true; 
                light.counter = 0; 

                if self.change_light {
                    self.current_light = (self.current_light + 1) % self.traffic_lights.len(); // Passer au feu suivant
                    self.change_light = false;
                }
            }
        }
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
