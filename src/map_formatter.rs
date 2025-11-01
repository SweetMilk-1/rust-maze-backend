use std::fmt;

use crate::map::{Cell, Map};

impl Cell {
    fn to_char(self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Start => 'i',
            Cell::End => 'O',
            Cell::Wall => '#',
            Cell::Path => '.',
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell.to_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
