#[cfg(test)]
mod tests {
    use crate::map::Map;
    use crate::map::Cell;

    #[test]
    fn test_parse_map() {
        let input = "# i\nO #\n".to_string();
        let map = Map::parse_from_string(input).unwrap();
        
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
        let mut map = Map::parse_from_string(input).unwrap();
        
        let found_path = map.find_and_mark_path();
        assert!(found_path);
        
        // Проверяем, что путь отмечен
        let output = map.to_string();
        assert!(output.contains('.'));
    }

    #[test]
    fn test_no_path() {
        let input = "#i#\n###\nO##\n".to_string();
        let mut map = Map::parse_from_string(input).unwrap();
        
        let found_path = map.find_and_mark_path();

        assert!(found_path == false);
        assert!(!map.to_string().contains('.'));
    }

    #[test]
    fn test_tor() {
        let input = "        \n  i     \n       O\n########".to_string();
        let mut map = Map::parse_from_string(input).unwrap();
        
        let found_path = map.find_and_mark_path();

        assert!(found_path == true);
        assert!(map.grid[2][0] == Cell::Path);
        assert!(map.grid[2][1] == Cell::Path);
        assert!(map.grid[2][2] == Cell::Path);
    }
}