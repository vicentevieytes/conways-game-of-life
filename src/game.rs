use crate::cell::Cell;

type Position = (usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Game {
    grid: Vec<Vec<Cell>>,
    alive_cells: Vec<Position>,
}

impl Game {
    pub fn from_alive_cells(dimensions: Position, alive_cells: &Vec<Position>) -> Self {
        let (w, h) = dimensions;
        let mut grid = vec![vec![Cell::new(); w]; h];

        for (i, j) in alive_cells.iter() {
            let cell_row = *i;
            let cell_col = *j;
            grid[cell_row][cell_col].give_life();
        }
        let alive_cells = alive_cells.clone();
        Game { grid, alive_cells }
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
        let game_1 = Game::from_alive_cells((5, 5), &alive_cells);

        let mut grid = game_grid_5_by_5();
        grid[1][1].give_life();
        grid[2][2].give_life();
        let game_2 = Game { grid, alive_cells };
        assert_eq!(game_1, game_2)
    }
}
