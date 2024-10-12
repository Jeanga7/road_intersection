extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

// Indicateur du type de mouvement de la voiture
pub enum Direction {
    Horizontal,
    Vertical,
}


pub struct Car {
    pub x: i32,
    pub y: i32,
    pub speed: i32,
    pub direction: Direction,  // Ajout d'un champ direction
}

impl Car {
    pub fn new(x: i32, y: i32, speed: i32, direction: Direction) -> Car {
        Car { x, y, speed, direction }
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::Horizontal => self.x += self.speed,  // Déplacement horizontal
            Direction::Vertical => self.y += self.speed,    // Déplacement vertical
        }
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let car_color = Color::RGB(255, 0, 0);
        canvas.set_draw_color(car_color);
        let car_rect = Rect::new(self.x, self.y, 30, 30);  // Taille des voitures (50x20)
        canvas.fill_rect(car_rect).unwrap();
    }
}
