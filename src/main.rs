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

    let mut running = false;

    loop {
        clear_background(WHITE);

        draw_grid(screen_width, screen_height, grid_size, width, height);
        draw_cells(&game, grid_size);
        handle_input(&mut game, grid_size, width, height, &mut running);

        if !running {
            draw_text("Click to toggle cells", 10.0, 20.0, 20.0, BLACK);
            draw_text("Spacebar to start/pause", 10.0, 40.0, 20.0, BLACK);
        } else {
            game.next();
        }

        next_frame().await;
    }
}

fn draw_grid(screen_width: f32, screen_height: f32, grid_size: f32, width: usize, height: usize) {
    // Draw horizontal lines
    for i in 0..=height as i32 {
        let y = i as f32 * grid_size;
        draw_line(0.0, y, screen_width, y, 1.0, BLACK);
    }

    // Draw vertical lines
    for i in 0..=width as i32 {
        let x = i as f32 * grid_size;
        draw_line(x, 0.0, x, screen_height, 1.0, BLACK);
    }
}

fn draw_cells(game: &Game, grid_size: f32) {
    // Paint cells based on the game state
    for cell in game.alive_cells() {
        let x = cell.0 as f32 * grid_size;
        let y = cell.1 as f32 * grid_size;
        draw_rectangle(x, y, grid_size, grid_size, BLACK);
    }
}

fn handle_input(game: &mut Game, grid_size: f32, width: usize, height: usize, running: &mut bool) {
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

    // Handle space key input
    if is_key_pressed(KeyCode::Space) {
        *running = !*running;
    }
}
