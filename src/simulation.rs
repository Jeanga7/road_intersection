use crate::road::*;
use crate::traffic_light::*;
use crate::vehicle::*;

pub struct Simulation {
    pub roads: Vec<Road>,
    pub traffic_lights: Vec<TrafficLight>,
    pub vehicles: Vec<Vehicle>,
}

impl Simulation {
    pub fn new() -> Self {
        let vertical_road1 = Road::new(350, 0, 50, 800, Orientation::Vertical);
        let vertical_road2 = Road::new(400, 0, 50, 800, Orientation::Vertical);

        let horizontal_road1 = Road::new(0, 350, 50, 800, Orientation::Horizontal);
        let horizontal_road2 = Road::new(0, 400, 50, 800, Orientation::Horizontal);

        Simulation {
            roads: vec![vertical_road1,vertical_road2, horizontal_road1, horizontal_road2],
            traffic_lights: vec![],
            vehicles: vec![],
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for road in &self.roads {
            road.draw(canvas);
        }
    }
}
