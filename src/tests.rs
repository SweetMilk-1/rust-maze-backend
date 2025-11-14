#[cfg(test)]
mod tests {
    use crate::map::{Cell, Map, Point};
    use crate::map_parser::ParsePointError;
    use std::str::FromStr;

    #[test]
    fn test_point_creation() {
        let point = Point { x: 1, y: 2 };
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn test_point_equality() {
        let point1 = Point { x: 1, y: 2 };
        let point2 = Point { x: 1, y: 2 };
        let point3 = Point { x: 3, y: 4 };
        
        assert_eq!(point1, point2);
        assert_ne!(point1, point3);
    }

    #[test]
    fn test_map_new() {
        let map = Map::new();
        assert_eq!(map.rows, 0);
        assert_eq!(map.cols, 0);
        assert!(map.grid.is_empty());
    }

    #[test]
    fn test_map_parsing_valid() {
        let input = "# #\n # ";
        let map = Map::from_str(input).unwrap();
        
        assert_eq!(map.rows, 2);
        assert_eq!(map.cols, 3);
        assert_eq!(map.grid[0][0], Cell::Wall);
        assert_eq!(map.grid[0][1], Cell::Empty);
        assert_eq!(map.grid[0][2], Cell::Wall);
        assert_eq!(map.grid[1][0], Cell::Empty);
        assert_eq!(map.grid[1][1], Cell::Wall);
        assert_eq!(map.grid[1][2], Cell::Empty);
    }

    #[test]
    fn test_map_parsing_empty() {
        let input = "";
        let result = Map::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsePointError);
    }

    #[test]
    fn test_map_parsing_invalid_character() {
        let input = "#x#";
        let result = Map::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsePointError);
    }

    #[test]
    fn test_map_parsing_uneven_rows() {
        let input = "###\n##";
        let result = Map::from_str(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParsePointError);
    }

    #[test]
    fn test_find_and_mark_path() {
        let mut map = Map::new();
        map.rows = 3;
        map.cols = 3;
        map.grid = vec![vec![Cell::Empty; 3]; 3];
        
        let start = Point { x: 0, y: 0 };
        let end = Point { x: 2, y: 2 };
        
        let result = map.find_and_mark_path(start, end);
        assert!(result);
        
        assert_eq!(map.grid[0][0], Cell::Start);
        assert_eq!(map.grid[2][2], Cell::End);
        // At least one path cell should be marked
        let path_cells_count = map.grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == Cell::Path)
            .count();
        assert!(path_cells_count > 0);
    }

    #[test]
    fn test_find_and_mark_path_no_path() {
        let mut map = Map::new();
        map.rows = 3;
        map.cols = 3;
        map.grid = vec![vec![Cell::Wall; 3]; 3];
        map.grid[0][0] = Cell::Empty;
        map.grid[2][2] = Cell::Empty;
        
        let start = Point { x: 0, y: 0 };
        let end = Point { x: 2, y: 2 };
        
        let result = map.find_and_mark_path(start, end);
        assert!(!result);
        
        // Grid should remain unchanged
        assert_eq!(map.grid[0][0], Cell::Empty);
        assert_eq!(map.grid[2][2], Cell::Empty);
    }

    #[test]
    fn test_map_display() {
        let mut map = Map::new();
        map.rows = 2;
        map.cols = 2;
        map.grid = vec![
            vec![Cell::Wall, Cell::Empty],
            vec![Cell::Empty, Cell::Wall],
        ];
        
        let display_output = format!("{}", map);
        let expected = "# \n #\n";
        assert_eq!(display_output, expected);
    }

    #[test]
    fn test_map_display_with_path() {
        let mut map = Map::new();
        map.rows = 2;
        map.cols = 2;
        map.grid = vec![
            vec![Cell::Start, Cell::Path],
            vec![Cell::Path, Cell::End],
        ];
        
        let display_output = format!("{}", map);
        let expected = "i.\n.O\n";
        assert_eq!(display_output, expected);
    }

    #[test]
    fn test_complex_path_finding() {
        // Create a more complex maze
        let input = "# # #\n     \n# # #";
        let mut map = Map::from_str(input).unwrap();
        
        let start = Point { x: 0, y: 1 };
        let end = Point { x: 2, y: 1 };
        
        let result = map.find_and_mark_path(start, end);
        assert!(result);
        
        // Verify start and end are marked
        assert_eq!(map.grid[0][1], Cell::Start);
        assert_eq!(map.grid[2][1], Cell::End);
    }
}