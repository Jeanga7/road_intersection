extern crate sdl2;
extern crate rand;


use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::traffic_light::TrafficLight;
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
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
        let mut rng = rand::thread_rng();
        Vehicle {
            x,
            y,
            speed: rng.gen_range(1.0..3.0),
            direction,
            turn: Self::random_turn_direction(),
            safe_distance: 50.0,
        }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::North => self.y -= self.speed as i32,
            Direction::South => self.y += self.speed as i32,
            Direction::East => self.x += self.speed as i32,
            Direction::West => self.x -= self.speed as i32,
        }
    }

    pub fn is_safe_distance(&self, other: &Vehicle) -> bool {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();
        distance >= self.safe_distance
    }

    pub fn stop_at_red_light(&mut self, traffic_light: &TrafficLight) {
        if !traffic_light.is_green {
            self.speed = 0.0;
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let color = self.get_color_based_on_route();
        canvas.set_draw_color(color);
        let vehicle_rect = Rect::new(self.x, self.y, 20, 10);
        canvas.fill_rect(vehicle_rect).unwrap();
    }

    fn get_color_based_on_route(&self) -> Color {
        match self.turn {
            TurnDirection::Left => Color::RGB(255, 0, 0),
            TurnDirection::Right => Color::RGB(255, 255, 0),
            TurnDirection::Straight => Color::RGB(0, 255, 0),
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