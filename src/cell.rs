#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn give_life(&mut self) {
        self.state = CellState::Alive;
    }

    pub fn kill(&mut self) {
        self.state = CellState::Dead;
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
        }
    }
    fn alive_cell() -> Cell {
        Cell {
            state: CellState::Alive,
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
}
