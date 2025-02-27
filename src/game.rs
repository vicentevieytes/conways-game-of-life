use crate::cell::Cell;
use crate::game_error::GameError;
use num::traits::Num;
use std::cmp::PartialOrd;
use std::collections::HashSet;
pub type Position = (usize, usize);

///Represents an instance of the Game of Life on a fixed size grid. Each of the cells can be killed or given life to
///and it's state can be modified to be the next iteration of the game by just calling `next()`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    grid: Vec<Vec<Cell>>,
    alive_cells: HashSet<Position>,
}

impl Game {
    /// Initializes a grid of height dimensions.0 and width dimensions.1
    pub fn of_size(dimensions: Position) -> Self {
        let (height, width) = dimensions;
        Game {
            grid: vec![vec![Cell::new(); width]; height],
            alive_cells: HashSet::new(),
        }
    }

    /// Initializes a game with set size and already alive cells
    pub fn from_size_and_cells(
        dimensions: Position,
        cells: &[Position],
    ) -> Result<Self, GameError> {
        let mut game = Game::of_size(dimensions);
        game.give_life_list(cells)?;
        Ok(game)
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
        self.alive_cells.remove(&pos);
        Ok(())
    }

    /// Gives life to a specific cell, returns error when out of bounds
    pub fn give_life(&mut self, pos: Position) -> Result<(), GameError> {
        self.check_in_bounds(pos)?;
        let cell = &mut self.grid[pos.0][pos.1];
        cell.give_life();
        self.alive_cells.insert(pos);
        Ok(())
    }

    /// Kills a list of cells
    pub fn kill_list(&mut self, list: &[Position]) -> Result<(), GameError> {
        for &pos in list.iter() {
            self.kill(pos)?;
        }
        Ok(())
    }

    /// Gives life to a list of cells
    pub fn give_life_list(&mut self, list: &[Position]) -> Result<(), GameError> {
        for &pos in list.iter() {
            self.give_life(pos)?;
        }
        Ok(())
    }

    /// Kills every cell in the board
    pub fn genocide(&mut self) {
        for pos in self.alive_cells().clone() {
            self.kill(pos)
                .expect("Shouldn't panic because all alive_cells should be in-bounds")
        }
    }
    /// Returns the vector of alive cells at the current iteration
    pub fn alive_cells(&self) -> &HashSet<Position> {
        &self.alive_cells
    }

    /// Returns the height and width of the grid.
    pub fn dimensions(&self) -> Position {
        (self.grid.len(), self.grid[0].len())
    }

    /// Returns a boolean indicating if the cell in the position is alive or not
    pub fn is_alive(&self, pos: Position) -> bool {
        self.alive_cells().contains(&pos)
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

        let _ = self.kill_list(&to_die);
        let _ = self.give_life_list(&to_live);
    }

    // Function to determine if a live cell should die
    fn should_die(&self, cell: &Cell, live_neighbors: usize) -> bool {
        cell.is_alive() && !(2..=3).contains(&live_neighbors)
    }

    // Function to determine if a dead cell should come to life
    fn should_live(&self, cell: &Cell, live_neighbors: usize) -> bool {
        !cell.is_alive() && live_neighbors == 3
    }

    // Returns the ammount of live neighbors of a certain position, used to decide wether a cell
    // lives or dies when next() is called
    fn live_neighbors(&self, position: Position) -> usize {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let height = self.dimensions().0 as isize;
        let width = self.dimensions().1 as isize;

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
        let (height, width) = self.dimensions();
        if !Self::in_bounds(pos.0, pos.1, height, width) {
            Err(GameError::OutOfBoundsGridAccess(
                (pos.0, pos.1),
                (height, width),
            ))
        } else {
            Ok(())
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

    #[test]
    fn test_initialize_game() {
        let alive_cells = vec![(1, 1), (2, 2)];
        let mut game_1 = Game::of_size((5, 5));
        game_1
            .give_life_list(&alive_cells)
            .expect("Game returned error on init");

        let mut grid = vec![vec![Cell::new(); 5]; 5];
        grid[1][1].give_life();
        grid[2][2].give_life();
        let game_2 = Game {
            grid,
            alive_cells: HashSet::from_iter(alive_cells),
        };

        assert_eq!(game_1, game_2)
    }

    #[test]
    fn test_initialize_game_from_size_and_cells() {
        let alive_cells = vec![(1, 1), (2, 2)];
        let game_1 = Game::from_size_and_cells((5, 5), &alive_cells).unwrap();

        let mut grid = vec![vec![Cell::new(); 5]; 5];
        grid[1][1].give_life();
        grid[2][2].give_life();
        let game_2 = Game {
            grid,
            alive_cells: HashSet::from_iter(alive_cells),
        };

        assert_eq!(game_1, game_2)
    }

    #[test]
    fn test_initialize_game_with_oob_alive_cells_should_fail() {
        let alive_cells = vec![(3, 3)];
        assert!(Game::from_size_and_cells((2, 2), &alive_cells).is_err());
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
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((1, 1)), 8)
    }

    #[test]
    fn test_upper_left_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(0, 1), (1, 0), (1, 1)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((0, 0)), 3)
    }

    #[test]
    fn test_upper_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(0, 1), (1, 0), (1, 1), (1, 2)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((0, 2)), 3)
    }

    #[test]
    fn test_upper_right_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(0, 2), (1, 2), (1, 3)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((0, 3)), 3)
    }

    #[test]
    fn test_right_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(1, 2), (2, 2), (3, 2)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((2, 3)), 3)
    }

    #[test]
    fn test_lower_right_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(2, 2), (2, 3), (3, 2)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((3, 3)), 3)
    }

    #[test]
    fn test_lower_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(2, 1), (2, 2), (2, 3)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((3, 2)), 3)
    }

    #[test]
    fn test_lower_left_corner_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(2, 0), (3, 1)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((3, 0)), 2)
    }

    #[test]
    fn test_left_side_live_neighbors() {
        let cells: Vec<Position> = Vec::from([(1, 1), (2, 1), (3, 1)]);
        let game = Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");
        assert_eq!(game.live_neighbors((2, 0)), 3)
    }
    #[test]
    fn test_underpopulation() {
        // Initialize a 4x4 grid with one cell alive
        let cells: Vec<Position> = Vec::from([(1, 1)]);
        let mut game =
            Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The single live cell should die due to underpopulation
        assert!(!game.is_alive((1, 1)));
    }

    #[test]
    fn test_survival() {
        // Initialize a 4x4 grid with three cells in a line (horizontal)
        let cells: Vec<Position> = Vec::from([(1, 0), (1, 1), (1, 2)]);
        let mut game =
            Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The three cells should rearrange to a vertical line
        assert!(game.is_alive((0, 1)));
        assert!(game.is_alive((1, 1)));
        assert!(game.is_alive((2, 1)));

        assert!(!game.is_alive((1, 0)));
        assert!(!game.is_alive((1, 2)));
    }

    #[test]
    fn test_overpopulation() {
        // Initialize a 4x4 grid with cells forming a small block
        let cells: Vec<Position> = Vec::from([(1, 1), (1, 2), (2, 1), (2, 2), (1, 0)]);
        let mut game =
            Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The cell at (1, 1) should die due to overpopulation
        assert!(!game.is_alive((1, 1)));
    }

    #[test]
    fn test_reproduction() {
        // Initialize a 4x4 grid with three cells in a corner
        let cells: Vec<Position> = Vec::from([(0, 1), (1, 0), (1, 1)]);
        let mut game =
            Game::from_size_and_cells((4, 4), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The dead cell at (0, 0) should come to life due to reproduction
        assert!(game.is_alive((0, 0)));
    }

    #[test]
    fn test_blinker_oscillator() {
        // Initialize a 5x5 grid with a blinker pattern (vertical line)
        let cells: Vec<Position> = Vec::from([(1, 2), (2, 2), (3, 2)]);
        let mut game =
            Game::from_size_and_cells((5, 5), &cells).expect("Game returned error on init");

        // Move to next generation
        game.next();

        // The blinker should turn into a horizontal line
        assert!(game.is_alive((2, 1)));
        assert!(game.is_alive((2, 2)));
        assert!(game.is_alive((2, 3)));

        assert!(!game.is_alive((1, 2)));
        assert!(!game.is_alive((3, 2)));
    }

    #[test]
    fn test_multiple_steps() {
        // Initialize a 5x5 grid with a blinker pattern (vertical line)
        let cells: Vec<Position> = Vec::from([(1, 2), (2, 2), (3, 2)]);
        let mut game =
            Game::from_size_and_cells((5, 5), &cells).expect("Game returned error on init");

        // First step: The blinker should turn into a horizontal line
        game.next();
        assert!(game.is_alive((2, 1)));
        assert!(game.is_alive((2, 2)));
        assert!(game.is_alive((2, 3)));

        assert!(!game.is_alive((1, 2)));
        assert!(!game.is_alive((3, 2)));

        // Second step: The blinker should return to a vertical line
        game.next();
        assert!(game.is_alive((1, 2)));
        assert!(game.is_alive((2, 2)));
        assert!(game.is_alive((3, 2)));

        assert!(!game.is_alive((2, 1)));
        assert!(!game.is_alive((2, 3)));

        // Third step: The blinker should turn into a horizontal line again
        game.next();
        assert!(game.is_alive((2, 1)));
        assert!(game.is_alive((2, 2)));
        assert!(game.is_alive((2, 3)));

        assert!(!game.is_alive((1, 2)));
        assert!(!game.is_alive((3, 2)));
    }
}
