use conways_game_of_life::cell::*;
use macroquad::prelude::*;

#[macroquad::main("Life")]

async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    //Cell::new() creates a dead cell
    let init_cell = Cell::new();

    let mut cells = vec![vec![init_cell.clone(); w]; h];
    for row in cells.iter_mut() {
        for cell in row.iter_mut() {
            if rand::gen_range(0, 5) == 0 {
                cell.give_life();
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
                if cell.is_alive() {
                    let x = col_index as f32 * grid_size;
                    let y = row_index as f32 * grid_size;
                    draw_rectangle(x, y, grid_size, grid_size, BLACK);
                }
            }
        }

        next_frame().await;
    }
}
