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
        let aggr_string = lines.join("\n");
        let map = Map::from_str(&aggr_string);
        match map {
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
