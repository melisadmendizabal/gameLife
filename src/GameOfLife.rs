


pub struct GameOfLife {
    width: usize,
    height: usize,
    board: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let mut board = vec![vec![false; width]; height];

        // Inicializamos el "Blinker"
        // Estado inicial horizontal: (X X X)
        board[height / 2][width / 2 - 1] = true;
        board[height / 2][width / 2] = true;
        board[height / 2][width / 2 + 1] = true;

        GameOfLife { width, height, board }
    }

    pub fn render(&self, framebuffer: &mut Framebuffer) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.board[y][x] {
                    framebuffer.set_pixel(x as u32, y as u32);
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