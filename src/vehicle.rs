pub struct Vehicle {
    pub x: f32,        
    pub y: f32,        
    pub speed: f32,    
    pub  color: Color,
    pub direction: Direction, 
}

pub enum Color{
    Red,
    Green,
    Yellow,
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Vehicle {
    pub fn new(x: f32, y: f32, speed: f32, color: Color, direction: Direction) -> Self {
        Vehicle {
            x,
            y,
            speed,
            color,
            direction,
        }
    }

    pub fn move_forward(&mut self) {
        
    }
}
