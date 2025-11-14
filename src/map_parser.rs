use std::str::FromStr;

use crate::map::{Cell, Map};

impl Cell {
    fn from_char(ch: char) -> Result<Self, String> {
        match ch {
            ' ' => Ok(Cell::Empty),
            '#' => Ok(Cell::Wall),
            _ => Err(format!("Illegal symbol: {}", ch)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl FromStr for Map {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        if lines.is_empty() {
            return Err(ParsePointError);
        }

        let mut map = Map::new();

        map.rows = lines.len();
        map.cols = lines[0].len();

        for line in lines.iter() {
            if  line.len() != map.cols {
                return Err(ParsePointError); 
            }

            let mut row_vec = Vec::with_capacity(map.cols);

            for ch in line.chars() {
                let cell = Cell::from_char(ch)
                    .map_err(|_| ParsePointError)?;
                row_vec.push(cell);
            }

            map.grid.push(row_vec);
        }

        Ok(map)
    }
}
