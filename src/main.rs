mod map;
mod map_formatter;
mod map_parser;

#[cfg(test)]
mod tests;

use map::Map;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<String>, _>>();
    if let Ok(lines) = input {
        let aggr_string = lines.join("\n");
        let map = Map::parse_from_string(aggr_string);
        match map {
            Ok(mut map) => {
                map.find_and_mark_path();
                println!("{}", map.to_string());
            }
            Err(err) => {
                eprintln!("Ошибка парсинга карты: {}", err);
                std::process::exit(1);
            }
        }
    }
}
