use std::fmt;

use crate::map::{Cell, Map};

impl Cell {
    pub fn to_char(self) -> char {
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
        for (i, row) in self.grid.iter().enumerate() {
            for cell in row {
                write!(f, "{}", cell.to_char())?;
            }
            // Не добавляем новую строку после последней строки
            if i < self.grid.len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
