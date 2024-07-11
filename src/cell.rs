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

    pub fn increase_alive_neighbors(&mut self) -> Result<(), CellError> {
        if self.alive_neighbors < 8 {
            self.alive_neighbors = self.alive_neighbors + 1;
            Ok(())
        } else {
            Err(CellError::AliveNeighborOverflow)
        }
    }

    pub fn decrease_alive_neighbors(&mut self) -> Result<(), CellError> {
        if self.alive_neighbors > 0 {
            self.alive_neighbors = self.alive_neighbors - 1;
            Ok(())
        } else {
            Err(CellError::AliveNeighborUnderflow)
        }
    }

    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }
}
