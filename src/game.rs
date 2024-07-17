use crate::cell::Cell;
use crate::game_error::GameError;
use num::traits::Num;
use std::cmp::PartialOrd;

pub type Position = (usize, usize);

///Represents an instance of the Game of Life, it's generated with an initial state of living
///cells, and it's state can be internally modified to be the next iteration of the game by just calling `next()`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    grid: Vec<Vec<Cell>>,
    alive_cells: Vec<Position>,
}

impl Game {
    /// Initializes a grid of height dimensions.0 and height dimensions.1
    pub fn of_size(dimensions: Position) -> Self {
        Game {
            grid: vec![vec![Cell::new(); dimensions.0]; dimensions.1],
            alive_cells: vec![],
        }
    }

    /// Kills the cell if it's alive, gives life to it if it's dead.
    pub fn toggle_cell(&mut self, pos: (usize, usize)) -> Result<(), GameError> {
        if self.alive_cells.contains(&pos) {
            self.kill(pos)
        } else {
            self.give_life(pos)
        }
    }

    /// Kills a specific cell, returns error when out of bounds
    pub fn kill(&mut self, pos: Position) -> Result<(), GameError> {
        self.check_in_bounds(pos)?;
        let cell = &mut self.grid[pos.0][pos.1];
        cell.kill();
        if let Some(index) = self.alive_cells.iter().position(|value| *value == pos) {
            self.alive_cells.swap_remove(index);
        }
        Ok(())
    }

    /// Gives life to a specific cell, returns error when out of bounds
    pub fn give_life(&mut self, pos: Position) -> Result<(), GameError> {
        self.check_in_bounds(pos)?;
        let cell = &mut self.grid[pos.0][pos.1];
        cell.give_life();
        if !self.alive_cells.contains(&pos) {
            self.alive_cells.push(pos)
        }
        Ok(())
    }

    /// Kills a list of cells
    pub fn kill_list(&mut self, list: &Vec<Position>) -> Result<(), GameError> {
        for &pos in list.iter() {
            self.kill(pos)?;
        }
        Ok(())
    }

    /// Gives life to a list of cells
    fn give_life_list(&mut self, list: &Vec<Position>) -> Result<(), GameError> {
        for &pos in list.iter() {
            self.give_life(pos)?;
        }
        Ok(())
    }

    /// Returns the vector of alive cells at the current iteration
    pub fn alive_cells(&self) -> &Vec<Position> {
        &self.alive_cells
    }

    /// Returns the width and height of the grid.
    pub fn dimensions(&self) -> Position {
        (self.grid.len(), self.grid[0].len())
    }

    /// Updates the internal state to represent the next step in the game according to the
    /// following rules:
    /// - Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
    /// - Any live cell with two or three live neighbours lives on to the next generation.
    /// - Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// - Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    pub fn next(&mut self) {
        let mut to_live = vec![];
        let mut to_die = vec![];
        //Iterate over all cells in the grid counting live_neighbors, cells which need to die or
        //be given life are pushed to the respective vectors.
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let live_neighbors = self.live_neighbors((i, j));

                if self.should_die(cell, live_neighbors) {
                    to_die.push((i, j));
                } else if self.should_live(cell, live_neighbors) {
                    to_live.push((i, j));
                }
            }
        }

        self.kill_list(&to_die);
        self.give_life_list(&to_live);

        self.alive_cells = to_live;
    }

    // Function to determine if a live cell should die
    fn should_die(&self, cell: &Cell, live_neighbors: usize) -> bool {
        cell.is_alive() && (live_neighbors < 2 || live_neighbors > 3)
    }

    // Function to determine if a dead cell should come to life
    fn should_live(&self, cell: &Cell, live_neighbors: usize) -> bool {
        !cell.is_alive() && live_neighbors == 3
    }

    fn live_neighbors(&self, position: Position) -> usize {
        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let width = self.grid[0].len() as isize;
        let height = self.grid.len() as isize;

        let (row, column) = position;
        let mut live_neighbors = 0;
        for &(dir_i, dir_j) in directions.iter() {
            let neighbor_row = row as isize + dir_i;
            let neighbor_column = column as isize + dir_j;

            if Self::in_bounds(neighbor_row, neighbor_column, height, width)
                && self.grid[neighbor_row as usize][neighbor_column as usize].is_alive()
            {
                live_neighbors += 1;
            }
        }
        live_neighbors
    }

    // Internal function to check if a Position is in bounds, returns error otherwise
    fn check_in_bounds(&self, pos: Position) -> Result<(), GameError> {
        if !Self::in_bounds(pos.0, pos.1, self.grid.len(), self.grid[0].len()) {
            return Err(GameError::OutOfBoundsGridAccess(
                (pos.0, pos.1),
                (self.grid.len(), self.grid[0].len()),
            ));
        } else {
            return Ok(());
        }
    }

    fn in_bounds<T>(row: T, column: T, height: T, width: T) -> bool
    where
        T: Num + PartialOrd,
    {
        row >= T::zero() && column >= T::zero() && row < height && column < width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_grid_5_by_5() -> Vec<Vec<Cell>> {
        vec![vec![Cell::new(); 5]; 5]
    }

    #[test]
    fn test_initialize_game() {
        let alive_cells = vec![(1, 1), (2, 2)];
        let game_1 = Game::from_cells((5, 5), &alive_cells).expect("Game returned error on init");

        let mut grid = game_grid_5_by_5();
        grid[1][1].give_life();
        grid[2][2].give_life();
        let game_2 = Game { grid, alive_cells };

        assert_eq!(game_1, game_2)
    }

    #[test]
    fn test_initialize_game_with_oob_alive_cells_should_fail() {
        let alive_cells = vec![(3, 3)];
        assert!(Game::from_cells((2, 2), &alive_cells).is_err());
    }

    #[test]
    fn test_8_live_neighbors() {
        let cells: Vec<Position> = Vec::from([
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((1, 1)), 8)
    }

    #[test]
    fn test_upper_left_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(0, 1), (1, 0), (1, 1)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((0, 0)), 3)
    }

    #[test]
    fn test_upper_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(0, 1), (1, 0), (1, 1), (1, 2)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((0, 2)), 3)
    }

    #[test]
    fn test_upper_right_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(0, 2), (1, 2), (1, 3)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((0, 3)), 3)
    }

    #[test]
    fn test_right_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(1, 2), (2, 2), (3, 2)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((2, 3)), 3)
    }

    #[test]
    fn test_lower_right_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(2, 2), (2, 3), (3, 2)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((3, 3)), 3)
    }

    #[test]
    fn test_lower_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(2, 1), (2, 2), (2, 3)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((3, 2)), 3)
    }

    #[test]
    fn test_lower_left_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(2, 0), (3, 1)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((3, 0)), 2)
    }

    #[test]
    fn test_left_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(1, 1), (2, 1), (3, 1)]);
        let game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((2, 0)), 3)
    }
    #[test]
    fn test_underpopulation() {
        // Initialize a 4x4 grid with one cell alive
        let cells: Vec<Position> = Vec::from([(1, 1)]);
        let mut game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The single live cell should die due to underpopulation
        assert!(!game.grid[1][1].is_alive());
    }

    #[test]
    fn test_survival() {
        // Initialize a 4x4 grid with three cells in a line (horizontal)
        let cells: Vec<Position> = Vec::from([(1, 0), (1, 1), (1, 2)]);
        let mut game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The three cells should rearrange to a vertical line
        assert!(game.grid[0][1].is_alive());
        assert!(game.grid[1][1].is_alive());
        assert!(game.grid[2][1].is_alive());

        assert!(!game.grid[1][0].is_alive());
        assert!(!game.grid[1][2].is_alive());
    }

    #[test]
    fn test_overpopulation() {
        // Initialize a 4x4 grid with cells forming a small block
        let cells: Vec<Position> = Vec::from([(1, 1), (1, 2), (2, 1), (2, 2), (1, 0)]);
        let mut game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The cell at (1, 1) should die due to overpopulation
        assert!(!game.grid[1][1].is_alive());
    }

    #[test]
    fn test_reproduction() {
        // Initialize a 4x4 grid with three cells in a corner
        let cells: Vec<Position> = Vec::from([(0, 1), (1, 0), (1, 1)]);
        let mut game = Game::from_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The dead cell at (0, 0) should come to life due to reproduction
        assert!(game.grid[0][0].is_alive());
    }

    #[test]
    fn test_blinker_oscillator() {
        // Initialize a 5x5 grid with a blinker pattern (vertical line)
        let cells: Vec<Position> = Vec::from([(1, 2), (2, 2), (3, 2)]);
        let mut game = Game::from_cells((5, 5), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The blinker should turn into a horizontal line
        assert!(game.grid[2][1].is_alive());
        assert!(game.grid[2][2].is_alive());
        assert!(game.grid[2][3].is_alive());

        assert!(!game.grid[1][2].is_alive());
        assert!(!game.grid[3][2].is_alive());
    }

    #[test]
    fn test_multiple_steps() {
        // Initialize a 5x5 grid with a blinker pattern (vertical line)
        let cells: Vec<Position> = Vec::from([(1, 2), (2, 2), (3, 2)]);
        let mut game = Game::from_cells((5, 5), &cells).expect("Game returned error on init");

        // First step: The blinker should turn into a horizontal line
        game.next();
        assert!(game.grid[2][1].is_alive());
        assert!(game.grid[2][2].is_alive());
        assert!(game.grid[2][3].is_alive());

        assert!(!game.grid[1][2].is_alive());
        assert!(!game.grid[3][2].is_alive());

        // Second step: The blinker should return to a vertical line
        game.next();
        assert!(game.grid[1][2].is_alive());
        assert!(game.grid[2][2].is_alive());
        assert!(game.grid[3][2].is_alive());

        assert!(!game.grid[2][1].is_alive());
        assert!(!game.grid[2][3].is_alive());

        // Third step: The blinker should turn into a horizontal line again
        game.next();
        assert!(game.grid[2][1].is_alive());
        assert!(game.grid[2][2].is_alive());
        assert!(game.grid[2][3].is_alive());

        assert!(!game.grid[1][2].is_alive());
        assert!(!game.grid[3][2].is_alive());
    }
}
