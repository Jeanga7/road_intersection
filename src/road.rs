extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Road {
    pub start_x: i32,        
    pub start_y: i32,        
    pub width: i32,          
    pub length: i32,         
    pub orientation: Orientation, 
}

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

        match self.orientation {
            Orientation::Vertical => {
                canvas.draw_line((self.start_x, self.start_y), (self.start_x, self.start_y + self.length)).unwrap();
                canvas.draw_line((self.start_x + self.width as i32, self.start_y), (self.start_x + self.width as i32, self.start_y + self.length)).unwrap();
            },
            Orientation::Horizontal => {
                canvas.draw_line((self.start_x, self.start_y), (self.start_x + self.length as i32, self.start_y)).unwrap();
                canvas.draw_line((self.start_x, self.start_y + self.width as i32), (self.start_x + self.length as i32, self.start_y + self.width as i32)).unwrap();
            }
        }    
        canvas.draw_line((350, 0), (350, 0 + 800)).unwrap();
        canvas.draw_line((400 + 0 as i32, 0), (400 + 0 as i32, 0 + 800)).unwrap();
        canvas.draw_line((450 + 0 as i32, 0), (450 + 0 as i32, 0 + 800)).unwrap();
    }    
}

