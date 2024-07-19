use conways_game_of_life::game::Game;
use macroquad::prelude::*;

#[macroquad::main("Game of Life")]
async fn main() {
    // Initialize the grid dimensions based on the screen size
    let screen_width = screen_width() * 4.0;
    let screen_height = screen_height() * 4.0;
    let grid_size = 20.0;
    let width = (screen_width / grid_size).ceil() as usize;
    let height = (screen_height / grid_size).ceil() as usize;

    // Initialize the game
    let mut game = Game::of_size((width, height));
    let mut running = false;

    // Camera offset and zoom
    let mut camera_offset = vec2(0.0, 0.0);
    let mut last_mouse_position = mouse_position();
    let mut zoom = 1.0;

    loop {
        clear_background(WHITE);

        handle_input(
            &mut game,
            grid_size,
            width,
            height,
            &mut running,
            &mut camera_offset,
            &mut last_mouse_position,
            &mut zoom,
        );

        draw_grid(
            screen_width,
            screen_height,
            grid_size,
            width,
            height,
            camera_offset,
            zoom,
        );
        draw_cells(&game, grid_size, camera_offset, zoom);

        if !running {
            draw_text("Click to toggle cells", 10.0, 20.0, 20.0, BLACK);
            draw_text("Spacebar to start/pause", 10.0, 40.0, 20.0, BLACK);
            draw_text("R to reset the board", 10.0, 60.0, 20.0, BLACK);
            draw_text(
                "Right-click and drag to move the grid",
                10.0,
                80.0,
                20.0,
                BLACK,
            );
            draw_text("Mouse wheel to zoom in/out", 10.0, 100.0, 20.0, BLACK);
        } else {
            game.next();
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
        next_frame().await;
    }
}

fn draw_grid(
    screen_width: f32,
    screen_height: f32,
    grid_size: f32,
    width: usize,
    height: usize,
    camera_offset: Vec2,
    zoom: f32,
) {
    let scaled_grid_size = grid_size * zoom;

    // Draw horizontal lines
    for i in 0..=height as i32 {
        let y = i as f32 * scaled_grid_size + camera_offset.y;
        draw_line(0.0, y, screen_width, y, 1.0, BLACK);
    }

    // Draw vertical lines
    for i in 0..=width as i32 {
        let x = i as f32 * scaled_grid_size + camera_offset.x;
        draw_line(x, 0.0, x, screen_height, 1.0, BLACK);
    }
}

fn draw_cells(game: &Game, grid_size: f32, camera_offset: Vec2, zoom: f32) {
    let scaled_grid_size = grid_size * zoom;

    // Paint cells based on the game state
    for cell in game.alive_cells() {
        let x = cell.0 as f32 * scaled_grid_size + camera_offset.x;
        let y = cell.1 as f32 * scaled_grid_size + camera_offset.y;
        draw_rectangle(x, y, scaled_grid_size, scaled_grid_size, BLACK);
    }
}

fn handle_input(
    game: &mut Game,
    grid_size: f32,
    width: usize,
    height: usize,
    running: &mut bool,
    camera_offset: &mut Vec2,
    last_mouse_position: &mut (f32, f32),
    zoom: &mut f32,
) {
    // Handle mouse input
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position = mouse_position();
        let cell_x = ((mouse_position.0 - camera_offset.x) / (grid_size * *zoom)).floor() as usize;
        let cell_y = ((mouse_position.1 - camera_offset.y) / (grid_size * *zoom)).floor() as usize;
        if cell_x < width && cell_y < height {
            game.toggle_cell((cell_x, cell_y))
                .expect("Error toggling cell");
        }
    }

    // Handle right mouse button dragging
    if is_mouse_button_down(MouseButton::Right) {
        let mouse_position = mouse_position();
        let delta = vec2(
            mouse_position.0 - last_mouse_position.0,
            mouse_position.1 - last_mouse_position.1,
        );
        *camera_offset += delta;
        *last_mouse_position = mouse_position;
    } else {
        *last_mouse_position = mouse_position();
    }

    // Handle space key input
    if is_key_pressed(KeyCode::Space) {
        *running = !*running;
    }

    // Handle reset key input
    if is_key_pressed(KeyCode::R) {
        game.genocide();
    }

    // Handle zooming
    let mouse_wheel = mouse_wheel().1;
    if mouse_wheel != 0.0 {
        *zoom *= if mouse_wheel > 0.0 { 1.1 } else { 0.9 };
    }
}
