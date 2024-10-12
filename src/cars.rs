extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Car {  // Rendre la structure publique
    pub x: i32,
    pub y: i32,
    pub speed: i32,
}

impl Car {
    pub fn new(x: i32, y: i32, speed: i32) -> Car {
        Car { x, y, speed }
    }

    pub fn update(&mut self) {  // Rendre la méthode update publique
        self.x += self.speed;  // Mouvement horizontal (sur une rue)
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {  // Rendre la méthode render publique
        let car_color = Color::RGB(255, 0, 0);
        canvas.set_draw_color(car_color);
        let car_rect = Rect::new(self.x, self.y, 50, 20);  // Dessiner un rectangle pour représenter la voiture
        canvas.fill_rect(car_rect).unwrap();
    }
}
