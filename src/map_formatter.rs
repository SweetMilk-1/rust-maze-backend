use crate::map::{Map, Cell};

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Start => 'i',
            Cell::End => 'O',
            Cell::Wall => '#',
            Cell::Path => '.'
        }
    }   
}

impl Map {
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.grid[row][col].to_char();
                result.push(ch);
            }
            result.push('\n');
        }

        result
    }
}