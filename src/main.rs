use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

#[macroquad::main("Life")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut cells = vec![vec![CellState::Dead; w]; h];
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

        next_frame().await;
    }
}
