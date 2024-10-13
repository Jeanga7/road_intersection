extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::vehicle::Direction;
use crate::vehicle::Vehicle;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LaneStage {
    Waiting,
    Crossing,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TrafficLight {
    pub is_green: bool,
    pub direction: Direction,
    pub timer: u32,
    pub stage: LaneStage,
}


impl TrafficLight {
    pub fn new(direction: Direction) -> Self {
        TrafficLight {
            is_green: false,
            direction,
            timer: 0,
            stage: LaneStage::Waiting,
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
            TrafficLight::new(Direction::East),
            TrafficLight::new(Direction::South),
            TrafficLight::new(Direction::West),
        ];

        Intersection {
            traffic_lights: lights,
            cycle_duration: 1000,
            current_phase: 0,
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        self.update_traffic_lights(vehicles);
    }

    fn update_traffic_lights(&mut self, vehicles: &[Vehicle]) {
        if self.traffic_lights.iter().any(|light| light.stage == LaneStage::Crossing) {
            return;
        }

        let mut next_cross_lane = None;
        let mut min_distance = f32::MAX;
        let mut max_vehicle_count = 0;

        for (i, light) in self.traffic_lights.iter().enumerate() {
            let waiting_vehicles: Vec<&Vehicle> = vehicles.iter()
                .filter(|v| v.direction == light.direction && self.is_vehicle_waiting(v))
                .collect();
            // println!("wainting {:?}", waiting_vehicles);

            if !waiting_vehicles.is_empty() {
                let closest_distance = waiting_vehicles.iter()
                    .map(|v| self.distance_to_stop_line(v))
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap();

                if closest_distance <= min_distance && waiting_vehicles.len() >= max_vehicle_count {
                    min_distance = closest_distance;
                    max_vehicle_count = waiting_vehicles.len();
                    next_cross_lane = Some(i);
                }
            }
        }

        // println!("wainting {:?}", next_cross_lane);

        if let Some(lane_index) = next_cross_lane {
            for (i, light) in self.traffic_lights.iter_mut().enumerate() {
                if i == lane_index {
                    light.is_green = true;
                    light.stage = LaneStage::Crossing;
                } else {
                    light.is_green = false;
                    light.stage = LaneStage::Waiting;
                }
            }
        }
    }

    fn is_vehicle_waiting(&self, vehicle: &Vehicle) -> bool {
        // let stop_line = match vehicle.direction {
        //     Direction::North => 400,
        //     Direction::South => 355,
        //     Direction::East => 355,
        //     Direction::West => 400,
        // };
        
        match vehicle.direction {
            Direction::North | Direction::South => /* (vehicle.y - stop_line).abs() < 50 &&*/ vehicle.speed == 0.0,
            Direction::East | Direction::West => /* (vehicle.x - stop_line).abs() < 50 &&*/ vehicle.speed == 0.0,
        } 
    }

    fn distance_to_stop_line(&self, vehicle: &Vehicle) -> f32 {
        let stop_line = match vehicle.direction {
            Direction::North => 400,
            Direction::South => 355,
            Direction::East => 355,
            Direction::West => 400,
        };
        
        match vehicle.direction {
            Direction::North => (vehicle.y - stop_line) as f32,
            Direction::South => (stop_line - vehicle.y) as f32,
            Direction::East => (stop_line - vehicle.x) as f32,
            Direction::West => (vehicle.x - stop_line) as f32,
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
