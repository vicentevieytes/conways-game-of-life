#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CellError {
    AliveNeighborOverflow,
    AliveNeighborUnderflow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
    pub alive_neighbours: i32,
}

impl Cell {
    pub fn give_life(&mut self) {
        self.state = CellState::Alive;
    }

    pub fn kill(&mut self) {
        self.state = CellState::Dead;
    }

    pub fn increase_neighbours(&mut self, num: i32) -> Result<(), CellError> {
        if self.alive_neighbours + num > 8 {
            Err(CellError::AliveNeighborOverflow)
        } else {
            self.alive_neighbours = self.alive_neighbours + num;
            Ok(())
        }
    }

    pub fn decrease_neighbours(&mut self, num: i32) -> Result<(), CellError> {
        if self.alive_neighbours - num < 0 {
            Err(CellError::AliveNeighborUnderflow)
        } else {
            self.alive_neighbours = self.alive_neighbours - num;
            Ok(())
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }

    pub fn neighbours(&self) -> i32 {
        self.alive_neighbours.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dead_cell() -> Cell {
        Cell {
            state: CellState::Dead,
            alive_neighbours: 0,
        }
    }
    fn alive_cell() -> Cell {
        Cell {
            state: CellState::Alive,
            alive_neighbours: 0,
        }
    }

    #[test]
    fn test_is_alive_on_alive_cell() {
        let alive_cell = alive_cell();
        assert!(alive_cell.is_alive());
    }

    #[test]
    fn test_is_alive_on_dead_cell() {
        let alive_cell = alive_cell();
        assert!(alive_cell.is_alive());
    }

    #[test]
    fn test_give_life_to_dead_cell() {
        let mut cell = dead_cell();
        cell.give_life();
        assert!(cell.is_alive());
    }

    #[test]
    fn test_kill_alive_cell() {
        let mut cell = alive_cell();
        cell.kill();
        assert!(!cell.is_alive());
    }

    #[test]
    fn test_increase_neighbours_within_limit() {
        let mut cell = dead_cell();
        let result = cell.increase_neighbours(1);
        assert!(result.is_ok());
        assert_eq!(cell.alive_neighbours, 1);
    }

    #[test]
    fn test_increase_neighbours_overflow() {
        let mut cell = dead_cell();
        let _ = cell.increase_neighbours(8);
        let result = cell.increase_neighbours(1);
        assert!(result.is_err());
        assert_eq!(cell.alive_neighbours, 8); // Should not increase beyond 8
    }

    #[test]
    fn test_decrease_neighbours_within_limit() {
        let mut cell = dead_cell();
        let _ = cell.increase_neighbours(5);
        let result = cell.decrease_neighbours(3);
        assert!(result.is_ok());
        assert_eq!(cell.alive_neighbours, 2);
    }

    #[test]
    fn test_decrease_neighbours_underflow() {
        let mut cell = dead_cell();
        let result = cell.decrease_neighbours(1);
        assert!(result.is_err());
        assert_eq!(cell.alive_neighbours, 0); // Should not decrease below 0
    }
}
