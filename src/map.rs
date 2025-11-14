use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub grid: Vec<Vec<Cell>>,
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
            rows: 0,
            cols: 0,
        }
    }

    pub fn find_and_mark_path(&mut self, start: Point, finish: Point) -> bool {
        let path = self.find_path(start, finish);
        match path {
            Some(path) => {
                self.mark_path(&path);
                true
            }
            None => false,
        }
    }

    fn find_path(&self, start: Point, finish: Point) -> Option<Vec<Point>> {
        let mut visited = vec![vec![false; self.cols]; self.rows];
        let mut parent = vec![vec![None; self.cols]; self.rows];
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited[start.x][start.y] = true;

        while let Some(current) = queue.pop_front() {
            if current == finish {
                // Восстанавливаем путь
                let mut path = Vec::new();
                let mut point = current;

                while point != start {
                    path.push(point);
                    if let Some(parent_point) = parent[point.x][point.y] {
                        point = parent_point;
                    } else {
                        break;
                    }
                }
                path.push(start); // Добавляем стартовую точку
                path.reverse();
                return Some(path);
            }

            for neighbor in self.get_neighbors(current) {
                if !visited[neighbor.x][neighbor.y] {
                    visited[neighbor.x][neighbor.y] = true;
                    parent[neighbor.x][neighbor.y] = Some(current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    fn mark_path(&mut self, path: &[Point]) {
        for (index, point) in path.iter().enumerate() {
            self.grid[point.x][point.y] = match index {
                0 => Cell::Start,
                i if i == path.len() - 1 => Cell::End,
                _ => Cell::Path,
            };
        }
    }

    pub fn get_neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dr, dc) in directions.iter() {
            // Преобразуем usize в isize для арифметических операций
            let row_isize: isize = point.x.try_into().unwrap_or(0);
            let col_isize: isize = point.y.try_into().unwrap_or(0);
            let rows_isize: isize = self.rows.try_into().unwrap_or(0);
            let cols_isize: isize = self.cols.try_into().unwrap_or(0);

            let new_row = (row_isize + dr).rem_euclid(rows_isize);
            let new_col = (col_isize + dc).rem_euclid(cols_isize);

            // Проверяем, что координаты неотрицательны и могут быть преобразованы в usize
            if new_row >= 0 && new_col >= 0 {
                // Преобразуем обратно в usize с проверкой
                if let Ok(new_row_usize) = usize::try_from(new_row) {
                    if let Ok(new_col_usize) = usize::try_from(new_col) {
                        // Проверяем, что координаты в пределах сетки
                        if new_row_usize < self.rows
                            && new_col_usize < self.cols
                            && self.grid[new_row_usize][new_col_usize] != Cell::Wall
                        {
                            neighbors.push(Point {
                                x: new_row_usize,
                                y: new_col_usize,
                            });
                        }
                    }
                }
            }
        }
        neighbors
    }
}

