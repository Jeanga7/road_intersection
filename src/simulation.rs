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
        let vertical_road1 = Road::new(350, 0, 50, 800, Orientation::Vertical);
        let vertical_road2 = Road::new(400, 0, 50, 800, Orientation::Vertical);
        let horizontal_road1 = Road::new(0, 350, 50, 800, Orientation::Horizontal);
        let horizontal_road2 = Road::new(0, 400, 50, 800, Orientation::Horizontal);

        Simulation {
            roads: vec![
                vertical_road1,
                vertical_road2,
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

        for i in 0..self.vehicles.len() {
            let vehicle = &mut self.vehicles[i];
            for light in &traffic_lights {
                if Self::is_vehicle_at_traffic_light(vehicle) {
                    vehicle.stop_at_red_light(light);
                }
            }

            let mut is_safe = true;
            for j in 0..vehicles_clone.len() {
                if i != j && !vehicle.is_safe_distance(&vehicles_clone[j]) {
                    is_safe = false;
                    break;
                }
            }
            if is_safe {
                vehicle.update();
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
            Direction::North => (375, 790),
            Direction::South => (425, 10),
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

    pub fn is_vehicle_at_traffic_light(vehicle: &Vehicle) -> bool {
        let distance = match vehicle.direction {
            Direction::North | Direction::South => (vehicle.y - 400).abs(),
            Direction::East | Direction::West => (vehicle.x - 400).abs(),
        };
        distance < 10
    }
}
