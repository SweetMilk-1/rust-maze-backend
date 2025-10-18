use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

pub struct Map {
    pub grid: Vec<Vec<Cell>>,
    pub start: Point,
    pub end: Point,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Wall,
    Empty,
    Start,
    End,
    Path,
}

impl Map {
    pub fn new() -> Self {
        Self {
            grid: Vec::new(),
            start: Point { row: 0, col: 0 },
            end: Point { row: 0, col: 0 },
            rows: 0,
            cols: 0,
        }
    }

    pub fn find_and_mark_path(&mut self) -> bool {
        let path = self.find_path();
        match path {
            Some(path) => {
                self.mark_path(&path);
                true
            }
            None => false
        }
    }

    fn find_path(&self) -> Option<Vec<Point>> {
        let mut visited = vec![vec![false; self.cols]; self.rows];
        let mut parent = vec![vec![None; self.cols]; self.rows];
        let mut queue = VecDeque::new();

        queue.push_back(self.start);
        visited[self.start.row][self.start.col] = true;

        while let Some(current) = queue.pop_front() {
            if current == self.end {
                // Восстанавливаем путь
                let mut path = Vec::new();
                let mut point = current;

                while point != self.start {
                    path.push(point);
                    if let Some(parent_point) = parent[point.row][point.col] {
                        point = parent_point;
                    } else {
                        break;
                    }
                }
                path.reverse();
                return Some(path);
            }

            for neighbor in self.get_neighbors(current) {
                if !visited[neighbor.row][neighbor.col] {
                    visited[neighbor.row][neighbor.col] = true;
                    parent[neighbor.row][neighbor.col] = Some(current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    pub fn get_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dr, dc) in directions.iter() {
            let new_row = (point.row as isize + dr).rem_euclid(self.rows as isize) as usize;
            let new_col = (point.col as isize + dc).rem_euclid(self.cols as isize) as usize;

            if self.grid[new_row][new_col] != Cell::Wall {
                neighbors.push(Point { row: new_row, col: new_col });
            }
        }

        neighbors
    }

    fn mark_path(&mut self, path: &[Point]) {
        for point in path {
            if self.grid[point.row][point.col] == Cell::Empty {
                self.grid[point.row][point.col] = Cell::Path;
            }
        }
    }
}
