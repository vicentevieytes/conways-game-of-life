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
    pub alive_neighbors: i32,
}

impl Cell {
    pub fn give_life(&mut self) {
        self.state = CellState::Alive;
    }

    pub fn kill(&mut self) {
        self.state = CellState::Dead;
    }

    pub fn increase_alive_neighbors(&mut self, num: i32) -> Result<(), CellError> {
        if self.alive_neighbors + num > 8 {
            Err(CellError::AliveNeighborOverflow)
        } else {
            self.alive_neighbors = self.alive_neighbors + num;
            Ok(())
        }
    }

    pub fn decrease_alive_neighbors(&mut self, num: i32) -> Result<(), CellError> {
        if self.alive_neighbors - num < 0 {
            Err(CellError::AliveNeighborUnderflow)
        } else {
            self.alive_neighbors = self.alive_neighbors - num;
            Ok(())
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dead_cell() -> Cell {
        Cell {
            state: CellState::Dead,
            alive_neighbors: 0,
        }
    }
    fn alive_cell() -> Cell {
        Cell {
            state: CellState::Alive,
            alive_neighbors: 0,
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
    fn test_increase_alive_neighbors_within_limit() {
        let mut cell = dead_cell();
        let result = cell.increase_alive_neighbors(1);
        assert!(result.is_ok());
        assert_eq!(cell.alive_neighbors, 1);
    }

    #[test]
    fn test_increase_alive_neighbors_overflow() {
        let mut cell = dead_cell();
        let _ = cell.increase_alive_neighbors(8);
        let result = cell.increase_alive_neighbors(1);
        assert!(result.is_err());
        assert_eq!(cell.alive_neighbors, 8); // Should not increase beyond 8
    }

    #[test]
    fn test_decrease_alive_neighbors_within_limit() {
        let mut cell = dead_cell();
        let _ = cell.increase_alive_neighbors(5);
        let result = cell.decrease_alive_neighbors(3);
        assert!(result.is_ok());
        assert_eq!(cell.alive_neighbors, 2);
    }

    #[test]
    fn test_decrease_alive_neighbors_underflow() {
        let mut cell = dead_cell();
        let result = cell.decrease_alive_neighbors(1);
        assert!(result.is_err());
        assert_eq!(cell.alive_neighbors, 0); // Should not decrease below 0
    }
}
