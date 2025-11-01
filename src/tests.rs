#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::map::Cell;
    use crate::map::Map;
    use crate::map::Point;

    #[test]
    fn test_parse_map() {
        let input = "# i\nO #\n".to_string();
        let map = Map::from_str(&input).unwrap();

        assert_eq!(map.rows, 2);
        assert_eq!(map.cols, 3);
        assert_eq!(map.start.row, 0);
        assert_eq!(map.start.col, 2);
        assert_eq!(map.end.row, 1);
        assert_eq!(map.end.col, 0);
    }

    #[test]
    fn test_path_finding() {
        let input = "#  \n i \n  O\n".to_string();
        let mut map = Map::from_str(&input).unwrap();

        let found_path = map.find_and_mark_path();
        assert!(found_path);

        // Проверяем, что путь отмечен
        let output = map.to_string();
        assert!(output.contains('.'));
    }

    #[test]
    fn test_no_path() {
        let input = "#i#\n###\nO##\n".to_string();
        let mut map = Map::from_str(&input).unwrap();

        let found_path = map.find_and_mark_path();

        assert!(found_path == false);
        assert!(!map.to_string().contains('.'));
    }

    #[test]
    fn test_tor() {
        let input = "        \n  i     \n       O\n########".to_string();
        let mut map = Map::from_str(&input).unwrap();

        let found_path = map.find_and_mark_path();
        assert!(found_path == true);
        assert!(go_path(&map, map.start, None));
    }

    fn go_path(map: &Map, current_point: Point, prev_point: Option<Point>) -> bool {
        let neigbours = map.get_neighbors_safe(current_point);
        if map.grid[current_point.row][current_point.col] == Cell::End {
            return true;
        }

        for n in &neigbours {
            if map.grid[n.row][n.col] != Cell::Path && map.grid[n.row][n.col] != Cell::End {
                continue;
            }

            if Some(*n) == prev_point {
                continue;
            }

            if go_path(map, *n, Some(current_point)) {
                return true;
            }
        }
        false
    }
}
