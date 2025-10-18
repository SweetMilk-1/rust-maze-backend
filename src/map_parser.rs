use crate::map::{Map, Point, Cell};

impl Cell {
    fn from_char(ch: char) -> Result<Self, String> {
        match ch {
             ' ' => Ok(Cell::Empty),
             'i' => Ok(Cell::Start),
             'O' => Ok(Cell::End),
             '#' => Ok(Cell::Wall),
             '.' => Ok(Cell::Path),
             _ => Err(format!("Unknown symbol: {}", ch))
        }
    }  
}

impl Map {
    pub fn parse_from_string(map_string: String) -> Result<Map, String> {
        let lines: Vec<&str> = map_string.lines().collect();

        if lines.is_empty() {
            return Err("Пустой ввод".to_string());
        }

        let mut map = Map::new();
        map.rows = lines.len();
        map.cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        for (row, line) in lines.iter().enumerate() {
            let mut row_vec = Vec::with_capacity(map.cols);
            for (col, ch) in line.chars().enumerate() {
                let mut cell = Cell::from_char(ch)?;
                match cell {
                    Cell::Start => map.start = Point { row, col },
                    Cell::End => map.end = Point { row, col },
                    Cell::Path => cell = Cell::Empty,
                    _ => {}
                }
                row_vec.push(cell);
            }

            while row_vec.len() < map.cols {
                row_vec.push(Cell::Empty);
            }
            map.grid.push(row_vec);
        }

        Ok(map)
    }
}
