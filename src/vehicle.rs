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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TurnDirection {
    Left,
    Right,
    Straight,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub speed: f32,
    pub direction: Direction,
    pub turn: TurnDirection,
    pub has_turned: bool,
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
            has_turned: false,
            safe_distance: 100.0,
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        let can_move = self.can_move_safely(vehicles);

        if can_move {
            if self.speed == 0.0 {
                self.speed = 1.0;
            }

            match self.direction {
                Direction::North => {
                    let virage = if self.turn == TurnDirection::Right {
                        355
                    } else {
                        405
                    };
                    if self.y > virage {
                        self.y -= self.speed as i32;
                    } else {
                        match self.turn {
                            TurnDirection::Left => {
                                self.x += self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Right => {
                                self.x -= self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Straight => self.y -= self.speed as i32,
                        }
                    }
                }

                Direction::South => {
                    let virage = if self.turn == TurnDirection::Left {
                        355
                    } else {
                        405
                    };
                    if self.y < virage {
                        self.y += self.speed as i32;
                    } else {
                        match self.turn {
                            TurnDirection::Left => {
                                self.x -= self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Right => {
                                self.x += self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Straight => self.y += self.speed as i32,
                        }
                    }
                }

                Direction::East => {
                    let virage = if self.turn == TurnDirection::Left {
                        355
                    } else {
                        405
                    };
                    if self.x < virage {
                        self.x += self.speed as i32;
                    } else {
                        match self.turn {
                            TurnDirection::Left => {
                                self.y += self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Right => {
                                self.y -= self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Straight => self.x += self.speed as i32,
                        }
                    }
                }

                Direction::West => {
                    let virage = if self.turn == TurnDirection::Left {
                        355
                    } else {
                        405
                    };
                    if self.x > virage {
                        self.x -= self.speed as i32;
                    } else {
                        match self.turn {
                            TurnDirection::Left => {
                                self.y += self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Right => {
                                self.y -= self.speed as i32;
                                self.has_turned = true;
                            }
                            TurnDirection::Straight => self.x -= self.speed as i32,
                        }
                    }
                }
            }
        } else {
            self.speed = (self.speed - 0.1).max(0.0);
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
        if self.direction != other.direction || self.has_turned != other.has_turned{
            return true;
        }

        let distance = match self.direction {
            Direction::North | Direction::South => (self.y - other.y).abs(),
            Direction::East | Direction::West => (self.x - other.x).abs(),
        };

        // println!("self {:?}, other {:?}", self.has_turned, other.has_turned);
        if distance != 0 {
            if !(distance >= self.safe_distance as i32) {
                // println!("self {:?}, other {:?}", self.direction, other.direction);
                // println!("in {}", "stop");
            }
            distance >= self.safe_distance as i32
        } else {
            true
        }
    }

    pub fn stop_at_red_light(&mut self, traffic_light: &TrafficLight) {
        if !traffic_light.is_green {
            // println!("direction {:?}, traffic_light: {:?}", self.direction, traffic_light);
            self.speed = 0.0;
        } else if self.speed == 0.0 {
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
