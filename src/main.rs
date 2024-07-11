use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CellError {
    AliveNeighborOverflow,
    AliveNeighborUnderflow,
    DeadCell,
    AliveCell,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Cell {
    pub state: CellState,
    pub alive_neighbors: i32,
}

impl Cell {
    pub fn give_life(&mut self) -> Result<(), CellError> {
        if self.state == CellState::Alive {
            Err(CellError::AliveCell)
        } else {
            self.state = CellState::Alive;
            Ok(())
        }
    }

    pub fn kill(&mut self) -> Result<(), CellError> {
        if self.state == CellState::Dead {
            Err(CellError::DeadCell)
        } else {
            self.state = CellState::Dead;
            Ok(())
        }
    }

    pub fn increase_alive_neighbors(&mut self) -> Result<(), CellError> {
        if self.alive_neighbors < 8 {
            self.alive_neighbors = self.alive_neighbors + 1;
            Ok(())
        } else {
            Err(CellError::AliveNeighborOverflow)
        }
    }

    pub fn decrease_alive_neighbors(&mut self) -> Result<(), CellError> {
        if self.alive_neighbors > 0 {
            self.alive_neighbors - 1;
            Ok(())
        } else {
            Err(CellError::AliveNeighborUnderflow)
        }
    }
}
#[macroquad::main("Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let init_cell = Cell {
        state: CellState::Dead,
        alive_neighbors: 0,
    };

    let mut cells = vec![vec![init_cell.clone(); w]; h];
    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            if rand::gen_range(0, 5) == 0 {
                *cell = CellState::Alive;
            }
        }
    }

    // Define grid parameters
    let grid_size = 20.0;
    let screen_width = screen_width();
    let screen_height = screen_height();
    let horizontal_lines = (screen_height / grid_size).ceil() as i32;
    let vertical_lines = (screen_width / grid_size).ceil() as i32;

    loop {
        clear_background(WHITE);

        // Draw horizontal lines
        for i in 0..=horizontal_lines {
            let y = i as f32 * grid_size;
            draw_line(0.0, y, screen_width, y, 1.0, BLACK);
        }

        // Draw vertical lines
        for i in 0..=vertical_lines {
            let x = i as f32 * grid_size;
            draw_line(x, 0.0, x, screen_height, 1.0, BLACK);
        }

        // Paint cells based on the cells matrix
        for (row_index, row) in cells.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell == CellState::Alive {
                    let x = col_index as f32 * grid_size;
                    let y = row_index as f32 * grid_size;
                    draw_rectangle(x, y, grid_size, grid_size, BLACK);
                }
            }
        }

        next_frame().await;
    }
}
