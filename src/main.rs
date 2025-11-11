mod map;
mod map_formatter;
mod map_parser;

#[cfg(test)]
mod tests;

use map::Map;
use std::{
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<String>, _>>();
    
    if let Ok(lines) = input {
        let combined_input = lines.join("\n");
        let map_result = Map::from_str(&combined_input);
        
        match map_result {
            Ok(mut map) => {
                map.find_and_mark_path();
                println!("{}", map);
            }
            Err(_) => {
                eprintln!("Ошибка парсинга карты");
                std::process::exit(1);
            }
        }
    }
}
