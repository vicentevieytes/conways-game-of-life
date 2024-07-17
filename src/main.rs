use conways_game_of_life::game::Game;
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    // Initialize the grid dimensions based on the screen size
    let screen_width = screen_width();
    let screen_height = screen_height();
    let grid_size = 20.0;
    let width = (screen_width / grid_size).ceil() as usize;
    let height = (screen_height / grid_size).ceil() as usize;

    // Initialize the game
    let mut game = Game::of_size((width, height));

    // Define grid parameters
    let horizontal_lines = height as i32;
    let vertical_lines = width as i32;

    let mut running = false;

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

        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            let cell_x = (mouse_position.0 / grid_size).floor() as usize;
            let cell_y = (mouse_position.1 / grid_size).floor() as usize;
            if cell_x < width && cell_y < height {
                game.toggle_cell((cell_x, cell_y))
                    .expect("Error toggling cell");
            }
        }
        // Handle buttons
        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }

        if !running {
            draw_text("Click to toggle cells", 10.0, 20.0, 20.0, BLACK);
            draw_text("Spacebar to start/pause", 10.0, 40.0, 20.0, BLACK);
            if is_key_pressed(KeyCode::Space) {
                game.next();
            }
        } else {
            game.next();
        }

        next_frame().await;
    }
}
