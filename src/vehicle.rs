extern crate rand;
extern crate sdl2;

use crate::traffic_light::TrafficLight;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, PartialEq)]
pub enum TurnDirection {
    Left,
    Right,
    Straight,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub speed: f32,
    pub direction: Direction,
    pub turn: TurnDirection,
    pub safe_distance: f32,
}

impl Vehicle {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Vehicle {
            x,
            y,
            speed: 1.0,
            direction,
            turn: Self::random_turn_direction(),
            safe_distance: 50.0,
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        let can_move = self.can_move_safely(vehicles);
        // println!("{}", can_move);
        let (coordinate, limit, forward, backward, perpendicular) = match self.direction {
            Direction::North => (&mut self.y, 355, -1, 1, &mut self.x),
            Direction::South => (&mut self.y, 405, 1, -1, &mut self.x),
            Direction::East => (&mut self.x, 405, 1, -1, &mut self.y),
            Direction::West => (&mut self.x, 355, -1, 1, &mut self.y),
        };

        if can_move {
            if (self.direction == Direction::North || self.direction == Direction::West) && *coordinate > limit
                || (self.direction == Direction::South || self.direction == Direction::East) && *coordinate < limit
            {
                *coordinate += forward * self.speed as i32;
            } else {
                // Handle turns
                match self.turn {
                    TurnDirection::Left => *perpendicular += self.speed as i32,
                    TurnDirection::Right => *perpendicular += backward * self.speed as i32,
                    TurnDirection::Straight => *coordinate += forward * self.speed as i32,
                }
            }
        } else {
            self.speed = 0.0;
        }
    }

    fn can_move_safely(&self, vehicles: &[Vehicle]) -> bool {
        vehicles.iter().all(|other| {
            if self as *const _ == other as *const _ {
                return true; 
            }
            self.is_safe_distance(other)
        })
    }

    pub fn is_safe_distance(&self, other: &Vehicle) -> bool {
        let distance = match self.direction {
            Direction::North | Direction::South => (self.y - other.y).abs(),
            Direction::East | Direction::West => (self.x - other.x).abs(),
        };
        println!("Direction de self: {:?}, Direction de other: {:?}", self.direction, other.direction);
        println!("Distance calculée: {}", distance);
    
        if distance != 0  {
            return distance >= self.safe_distance as i32
        }
        true
    }
    

    pub fn stop_at_red_light(&mut self, traffic_light: &TrafficLight) {
        println!("{}", "red stop");
        println!("state {}", traffic_light.is_green);
        println!("direction {:?}", traffic_light.direction);
        if !traffic_light.is_green {
            println!("{}", "is red");
            self.speed = 0.0;
        } else if self.speed == 0.0 {
            println!("{}", "is green");
            self.speed = 1.0;
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = self.get_color_based_on_route();
        canvas.set_draw_color(color);
        let vehicle_rect = match self.direction {
            Direction::East | Direction::West => Rect::new(self.x, self.y - 20, 40, 40),
            Direction::North | Direction::South => Rect::new(self.x - 20, self.y, 40, 40),
        };
        canvas.fill_rect(vehicle_rect).unwrap();
    }

    fn get_color_based_on_route(&self) -> Color {
        match self.turn {
            TurnDirection::Left => Color::RGB(255, 0, 0), // rouge a gauche
            TurnDirection::Right => Color::RGB(255, 255, 0), //jaune - a droite
            TurnDirection::Straight => Color::RGB(0, 255, 0), //vert - tout droit
        }
    }

    fn random_turn_direction() -> TurnDirection {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => TurnDirection::Left,
            1 => TurnDirection::Right,
            _ => TurnDirection::Straight,
        }
    }
}
