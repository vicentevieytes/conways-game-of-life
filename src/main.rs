use conways_game_of_life::game::{Game, Position};
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    // Initialize the grid dimensions based on the screen size
    let screen_width = screen_width();
    let screen_height = screen_height();
    let grid_size = 20.0;
    let width = (screen_width / grid_size).ceil() as usize;
    let height = (screen_height / grid_size).ceil() as usize;

    // Create initial living cells
    let mut initial_living_cells = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if rand::gen_range(0, 5) == 0 {
                initial_living_cells.push((x, y));
            }
        }
    }

    // Initialize the game
    let mut game = match Game::from_cells((width, height), &initial_living_cells) {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Failed to initialize the game: {:?}", e);
            return;
        }
    };

    // Define grid parameters
    let horizontal_lines = height as i32;
    let vertical_lines = width as i32;

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

        // Paint cells based on the game state
        for cell in game.alive_cells() {
            let x = cell.0 as f32 * grid_size;
            let y = cell.1 as f32 * grid_size;
            draw_rectangle(x, y, grid_size, grid_size, BLACK);
        }

        // Update the game state for the next frame
        game.next();

        next_frame().await;
    }
}
