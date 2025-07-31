use raylib::prelude::*;
use raylib::color::Color;
mod framebuffer;
use crate::framebuffer::Framebuffer;
// mod line;
// use line::line;
use std::thread;
use std::time::Duration;

pub struct GameOfLife {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            width,
            height,
            board: vec![vec![false; width]; height],
        }
    }


    pub fn create_glider(&mut self, x: usize, y: usize) {
        // Colocamos el patrón de Glider en las coordenadas (x, y)
        if x + 3 < self.width && y + 3 < self.height {
            self.board[y][x + 1] = true;   // O
            self.board[y + 1][x + 2] = true; // O
            self.board[y + 2][x] = true;   // O
            self.board[y + 2][x + 1] = true; // O
            self.board[y + 2][x + 2] = true; // O
        }
    }

    pub fn create_lwss(&mut self, x: usize, y: usize) {
        if x + 4 < self.width && y + 3 < self.height {
            self.board[y][x + 1] = true;
            self.board[y][x + 2] = true;
            self.board[y][x + 3] = true;

            self.board[y + 1][x] = true;
            self.board[y + 1][x + 4] = true;

            self.board[y + 2][x + 4] = true;

            self.board[y + 3][x] = true;
            self.board[y + 3][x + 3] = true;
        }
    }


    pub fn create_blinker(&mut self, x: usize, y: usize) {
        // Colocamos el patrón de Glider en las coordenadas (x, y)
        if x + 1 < self.width && y < self.height {
            self.board[y][x - 1] = true;
            self.board[y][x] = true;
            self.board[y][x + 1] = true;
        }
    }

    pub fn create_block(&mut self, x: usize, y: usize) {
        // Colocamos el patrón de Glider en las coordenadas (x, y)
        if x + 2 < self.width && y + 2 < self.height {
            self.board[y][x] = true;
            self.board[y][x + 1] = true;
            self.board[y][x] = true;
            self.board[y + 1][x] = true;
        }
    }

    pub fn create_beacon(&mut self, x: usize, y: usize) {
        // Colocamos el patrón de Glider en las coordenadas (x, y)
        if x + 4 < self.width && y + 4 < self.height {
            self.board[y][x] = true;
            self.board[y][x + 1] = true;
            self.board[y][x] = true;
            self.board[y + 1][x] = true;

            self.board[y + 2][x + 2] = true;
            self.board[y + 3][x + 3] = true;
            self.board[y + 2][x + 3] = true;
            self.board[y + 3][x + 2] = true;
        }
    }

    pub fn create_pulsar(&mut self, x: usize, y: usize) {
    // Coordenadas relativas para una de las 4 esquinas del pulsar
        let offsets = [
            // Parte superior horizontal
            (2, 0), (3, 0), (4, 0),
            (0, 2), (0, 3), (0, 4),

            // Parte inferior horizontal
            (2, 5), (3, 5), (4, 5),
            (5, 2), (5, 3), (5, 4),
        ];

        // Aplica la forma original y sus 3 rotaciones/reflejos
        let centers = [
            (x + 2, y + 2),             // cuadrante superior izquierdo
            (x + 8, y + 2),             // cuadrante superior derecho
            (x + 2, y + 8),             // cuadrante inferior izquierdo
            (x + 8, y + 8),             // cuadrante inferior derecho
        ];

        for &(cx, cy) in &centers {
            for &(dx, dy) in &offsets {
                let px = cx + dx;
                let py = cy + dy;
                if px < self.width && py < self.height {
                    self.board[py][px] = true;
                }
            }
        }
    }







    pub fn render(&self, framebuffer: &mut Framebuffer) {
        let cell_size = 5; 
        framebuffer.set_curret_color(Color::new(165, 82, 168,255));
        

        for y in 0..self.height {
            for x in 0..self.width {
                if self.board[y][x] {
                    for dy in 0..cell_size {
                        for dx in 0..cell_size {
                            // Pinta píxeles en la posición escalada:
                            framebuffer.set_pixel(
                                (x * cell_size + dx) as u32,
                                (y * cell_size + dy) as u32,
                                
                                
                            );
                        }
                    }
                }
            }
        }
    }
    

    // Función para actualizar el tablero
     pub fn update(&mut self) {
        let mut new_board = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let live_neighbors = self.count_live_neighbors(x, y);

                if self.board[y][x] {
                    if live_neighbors == 2 || live_neighbors == 3 {
                        new_board[y][x] = true; // La célula sobrevive
                    }
                } else if live_neighbors == 3 {
                    new_board[y][x] = true; // La célula se reproduce
                }
            }
        }

        self.board = new_board;
    }

    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue; // Ignorar la célula misma
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                    if self.board[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }

        count
    }

}




fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Game of live")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::new(87,23,78,255));
    let mut game_of_life = GameOfLife::new(framebuffer_width as usize, framebuffer_height as usize);

    // Grupo 1
  

    
    

    game_of_life.create_beacon(50, 50);


    game_of_life.create_block(10, 30);
    game_of_life.create_block(50, 30);
    game_of_life.create_block(90, 30);
    game_of_life.create_block(130, 30);

    game_of_life.create_block(10, 60);
    game_of_life.create_block(50, 60);
    game_of_life.create_block(90, 60);
    game_of_life.create_block(130, 60);

    game_of_life.create_block(10, 90);
    game_of_life.create_block(50, 90);
    game_of_life.create_block(90, 90);
    game_of_life.create_block(130, 90);
   

    //blinckr
    game_of_life.create_blinker(25, 30);
    game_of_life.create_blinker(65, 30);
    game_of_life.create_blinker(105, 30);
    game_of_life.create_blinker(145, 30);

    game_of_life.create_blinker(25, 60);
    game_of_life.create_blinker(65, 60);
    game_of_life.create_blinker(105, 60);
    game_of_life.create_blinker(145, 60);

    game_of_life.create_blinker(25, 90);
    game_of_life.create_blinker(65, 90);
    game_of_life.create_blinker(105, 90);
    game_of_life.create_blinker(145, 90);

    //glider
    game_of_life.create_glider(35, 105);
    game_of_life.create_glider(35, 75);
    game_of_life.create_glider(35, 45);
    game_of_life.create_glider(35, 15);

    game_of_life.create_glider(75, 105);
    game_of_life.create_glider(75, 75);
    game_of_life.create_glider(75, 45);
    game_of_life.create_glider(75, 15);

    game_of_life.create_glider(115, 105);
    game_of_life.create_glider(115, 75);
    game_of_life.create_glider(115, 45);
    game_of_life.create_glider(115, 15);


    //mas cosas
    game_of_life.create_lwss(35, 95);
    game_of_life.create_lwss(35, 65);
    game_of_life.create_lwss(35, 35);
    game_of_life.create_lwss(35, 5);

    game_of_life.create_lwss(75, 95);
    game_of_life.create_lwss(75, 65);
    game_of_life.create_lwss(75, 35);
    game_of_life.create_lwss(75, 5);

    game_of_life.create_lwss(115, 95);
    game_of_life.create_lwss(115, 65);
    game_of_life.create_lwss(115, 35);
    game_of_life.create_lwss(115, 5);

    
    //pulsadores
    game_of_life.create_pulsar(90, 100);
    game_of_life.create_pulsar(10, 100);
    game_of_life.create_pulsar(50, 100);
    game_of_life.create_pulsar(130, 100);

    game_of_life.create_pulsar(90, 70);
    game_of_life.create_pulsar(10, 70);
    game_of_life.create_pulsar(50, 70);
    game_of_life.create_pulsar(130, 70);

    game_of_life.create_pulsar(90, 40);
    game_of_life.create_pulsar(10, 40);
    game_of_life.create_pulsar(50, 40);
    game_of_life.create_pulsar(130, 40);

    game_of_life.create_pulsar(90, 10);
    game_of_life.create_pulsar(10, 10);
    game_of_life.create_pulsar(50, 10);
    game_of_life.create_pulsar(130, 10);

    



    //controlar tiempos de actualización
    let mut last_update = std::time::Instant::now();
    let update_interval = Duration::from_millis(500);


    framebuffer.set_background_color(Color::new(87,23,78,255));

    while !window.window_should_close() {
        // Si presionamos la tecla SPACE, actualizar el tablero
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game_of_life.update();
        }

        // Si ha pasado el tiempo de actualización, actualizar automáticamente
        if last_update.elapsed() > update_interval {
            game_of_life.update();
            last_update = std::time::Instant::now();
        }

        // Limpiar el framebuffer y renderizar el estado actual
        framebuffer.clear();
        game_of_life.render(&mut framebuffer);

        // Mostrar el framebuffer en la ventana
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // Controlar el FPS
        thread::sleep(Duration::from_millis(4));
    }

    // let mut translate_x = 0.0;
    // let mut translate_y = 0.0;

    // let mut direction_x = 1.0;
    // let mut direction_y = 1.0;
    // let speed = 2.0;

    // while !window.window_should_close() {
    //     if translate_x + 50.0 > framebuffer_width as f32 || translate_x < 0.0 {
    //         direction_x = - direction_x;
    //     }

    //     if translate_y + 50.0 > framebuffer_height as f32 || translate_y < 0.0 {
    //         direction_y = - direction_y;
    //     }

    //     translate_x += direction_x * speed;
    //     translate_y += direction_y * speed;
      

    //     framebuffer.clear();

    //     render(&mut framebuffer, translate_x, translate_y);

    //     // Si se presiona F12, hacer un screenshot
    //     if window.is_key_pressed(KeyboardKey::KEY_F12) {
    //         framebuffer.render_to_file("screenshot.png");
    //         println!("Screenshot guardado como screenshot.png");
    //     }

    //     framebuffer.swap_buffers(&mut window, &raylib_thread);

    //     thread::sleep(Duration::from_millis(16));

    
}

// fn render(
//     framebuffer:&mut Framebuffer,
//     translate_x: f32,
//     translate_y: f32,
// ) {
//     framebuffer.set_background_color(Color::GREEN);
//     line(
//         framebuffer,
//         Vector2::new(50.0 + translate_x, 50.0 +translate_y),
//         Vector2::new(350.0 + translate_x, 350.0 +translate_y),
//     );

//     framebuffer.set_background_color(Color::RED);
//     line(
//         framebuffer,
//         Vector2::new(350.0 + translate_x, 50.0 +translate_y),
//         Vector2::new(50.0 + translate_x, 350.0 +translate_y),
//     );
// }
