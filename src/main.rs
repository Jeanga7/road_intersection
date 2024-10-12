extern crate sdl2;
mod cars;  // Import du module cars

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use cars::Car;  // Import de la structure Car depuis le module cars

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Lignes centrées et voitures", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Dimensions de la fenêtre
    let window_width = 800;
    let window_height = 600;

    // Coordonnées pour les lignes horizontales et verticales décalées de 50 pixels
    let horizontal_y1 = window_height / 2 - 50; // Ligne horizontale 1 (au-dessus du centre)
    let horizontal_y2 = window_height / 2 + 50; // Ligne horizontale 2 (en-dessous du centre)
    let vertical_x1 = window_width / 2 - 50;    // Ligne verticale 1 (à gauche du centre)
    let vertical_x2 = window_width / 2 + 50;    // Ligne verticale 2 (à droite du centre)

    // Coordonnées pour la ligne centrale (milieu de l'écran)
    let center_x = window_width / 2;
    let center_y = window_height / 2;

    // Créer une liste de voitures
    let mut cars = vec![
        Car::new(0, horizontal_y1 - 25, 5),  // Voiture 1 sur la ligne horizontale 1
        Car::new(0, horizontal_y2 - 25, 3),  // Voiture 2 sur la ligne horizontale 2
    ];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Définir la couleur de fond (noir) et effacer l'écran
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Définir la couleur des lignes (blanc)
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Tracer les lignes horizontales et verticales
        canvas.draw_line((0, horizontal_y1), (window_width, horizontal_y1)).unwrap();
        canvas.draw_line((0, horizontal_y2), (window_width, horizontal_y2)).unwrap();
        canvas.draw_line((vertical_x1, 0), (vertical_x1, window_height)).unwrap();
        canvas.draw_line((vertical_x2, 0), (vertical_x2, window_height)).unwrap();
        canvas.draw_line((0, center_y), (window_width, center_y)).unwrap();
        canvas.draw_line((center_x, 0), (center_x, window_height)).unwrap();

        // Mettre à jour et dessiner chaque voiture
        for car in &mut cars {
            car.update();  // Mise à jour de la position de la voiture
            car.render(&mut canvas);  // Dessiner la voiture sur le canvas

            // Réinitialiser la position de la voiture si elle sort de l'écran
            if car.x > window_width {
                car.x = -50;  // Réinitialiser à gauche de l'écran
            }
        }

        // Présenter le rendu (afficher les lignes et les voitures)
        canvas.present();

        // Limiter à 60 images par seconde
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
