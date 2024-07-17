use crate::game::Position;
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum GameError {
    OutOfBoundsGridAccess(Position, Position),
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attemped access to an out of bounds position")
    }
}
