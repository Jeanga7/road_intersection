extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Copy, Clone, PartialEq)]
pub struct Road {
    pub start_x: i32,        
    pub start_y: i32,        
    pub width: i32,          
    pub length: i32,         
    pub orientation: Orientation, 
}

#[derive(Copy, Clone, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Road {
    pub fn new(start_x: i32, start_y: i32, width: i32, length: i32, orientation: Orientation) -> Self {
        Road {
            start_x,
            start_y,
            width,
            length,
            orientation,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(50, 50, 50)); 
    
        let road_rect = match self.orientation {
            Orientation::Vertical => Rect::new(self.start_x, self.start_y, self.width as u32, self.length as u32),
            Orientation::Horizontal => Rect::new(self.start_x, self.start_y, self.length as u32, self.width as u32),
        };
    
        canvas.fill_rect(road_rect).unwrap();
    
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    
        // Horizontal lines
        if let Orientation::Horizontal = self.orientation {
            canvas.draw_line((self.start_x, self.start_y), (self.start_x + self.length as i32, self.start_y)).unwrap();
            canvas.draw_line((self.start_x, self.start_y + self.width as i32), (self.start_x + self.length as i32, self.start_y + self.width as i32)).unwrap();
        }
    
        // Vertical lines
        for x in [350, 400, 450].iter() {
            canvas.draw_line((*x, 0), (*x, 800)).unwrap();
        }
    } 
}

