extern crate rand;
use crate::road::*;
use crate::traffic_light::*;
use crate::vehicle::*;
// Remove the unused import
// use rand::Rng;

#[derive(Clone, PartialEq)]
pub struct Simulation {
    pub roads: Vec<Road>,
    pub intersection: Intersection,
    pub vehicles: Vec<Vehicle>,
}

impl Simulation {
    pub fn new() -> Self {
        let vertical_road1 = Road::new(400, 0, 50, 800, Orientation::Vertical);
        let vertical_road2 = Road::new(350, 0, 50, 800, Orientation::Vertical);
        let horizontal_road1 = Road::new(0, 400, 50, 800, Orientation::Horizontal);
        let horizontal_road2 = Road::new(0, 350, 50, 800, Orientation::Horizontal);

        Simulation {
            roads: vec![
                vertical_road2,
                vertical_road1,
                horizontal_road1,
                horizontal_road2,
            ],
            intersection: Intersection::new(),
            vehicles: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.intersection.update();
        let traffic_lights = self.intersection.traffic_lights.clone();
        let vehicles_clone = self.vehicles.clone();
    
        let mut safety_checks = Vec::new();
        for (i, vehicle) in self.vehicles.iter().enumerate() {
            let relevant_light = traffic_lights
                .iter()
                .find(|&light| light.direction == vehicle.direction);
    
            let mut is_safe = true;
            if let Some(light) = relevant_light {
                if Self::is_vehicle_at_traffic_light(vehicle, light) {
                    is_safe = false;
                }
            }
    
            if is_safe {
                for (j, other_vehicle) in vehicles_clone.iter().enumerate() {
                    if i != j && !vehicle.is_safe_distance(other_vehicle) {
                        is_safe = false;
                        break;
                    }
                }
            }
            
            safety_checks.push(is_safe);
        }
    
        for (vehicle, &is_safe) in self.vehicles.iter_mut().zip(safety_checks.iter()) {
            if is_safe {
                Self::update_vehicle_priority(vehicle, &vehicles_clone);
                vehicle.update(&vehicles_clone);
            }
        }
    
        self.vehicles.retain(|v| v.x >= 0 && v.x <= 800 && v.y >= 0 && v.y <= 800);
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for road in &self.roads {
            road.draw(canvas);
        }
        self.intersection.draw(canvas);
        for vehicle in &self.vehicles {
            vehicle.render(canvas);
        }
    }

    pub fn spawn_vehicle(&mut self, direction: Direction) {
        let (x, y) = match direction {
            Direction::North => (425, 790),
            Direction::South => (375, 10),
            Direction::East => (10, 425),
            Direction::West => (790, 375),
        };

        let new_vehicle = Vehicle::new(x, y, direction);

        if self.is_safe_to_spawn(&new_vehicle) {
            self.vehicles.push(new_vehicle);
        }
    }

    fn is_safe_to_spawn(&self, new_vehicle: &Vehicle) -> bool {
        for vehicle in &self.vehicles {
            if vehicle.direction == new_vehicle.direction {
                let distance = match new_vehicle.direction {
                    Direction::North | Direction::South => (vehicle.y - new_vehicle.y).abs(),
                    Direction::East | Direction::West => (vehicle.x - new_vehicle.x).abs(),
                };
                if distance < 100 {
                    return false;
                }
            }
        }
        true
    }

    pub fn update_vehicle_priority(vehicle: &mut Vehicle, other_vehicles: &[Vehicle]) {
        for other_vehicle in other_vehicles {
            if other_vehicle != vehicle && !vehicle.is_safe_distance(other_vehicle) {
                match (vehicle.direction, other_vehicle.direction) {
                    (Direction::West, Direction::North)
                    | (Direction::North, Direction::East)
                    | (Direction::East, Direction::South)
                    | (Direction::South, Direction::West) => {
                        vehicle.speed = 0.0;
                        return;
                    }
                    (dir1, dir2) if dir1 == dir2 => {
                        continue;
                    }
                    _ => {
                        vehicle.speed = 1.0;
                    }
                }
            }
        }
    }

    pub fn is_vehicle_at_traffic_light(vehicle: &Vehicle, traffic_light: &TrafficLight) -> bool {
        match vehicle.direction {
            Direction::North => {
                (vehicle.y - 400).abs() < 55 && vehicle.direction == traffic_light.direction
            }
            Direction::South => {
                // println!("{}",  (vehicle.y - 350).abs() < 55 && vehicle.direction == traffic_light.direction);
                (vehicle.y - 355).abs() < 50 && vehicle.direction == traffic_light.direction
            }
            Direction::East => {
                (vehicle.x - 355).abs() < 50 && vehicle.direction == traffic_light.direction
            }
            Direction::West => {
                (vehicle.x - 400).abs() < 55 && vehicle.direction == traffic_light.direction
            }
        }
    }
}
