// use num_enum::TryFromPrimitive;
use std::fmt;
use serde::{Serialize, Deserialize};

pub type EntityID = u64;
pub type Name = String;

#[repr(u8)]
// #[derive(Debug, Clone, PartialEq, Eq, Hash, TryFromPrimitive, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North = 0,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "north"),
            Direction::Northeast => write!(f, "northeast"),
            Direction::East => write!(f, "east"),
            Direction::Southeast => write!(f, "southeast"),
            Direction::South => write!(f, "south"),
            Direction::Southwest => write!(f, "southwest"),
            Direction::West => write!(f, "west"),
            Direction::Northwest => write!(f, "northwest"),
        }
    }
}

