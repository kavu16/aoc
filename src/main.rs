use std::fs;

mod puzzles;

use crate::puzzles::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {panic!("Not enough args!")},
        _ => {
            let day = args[1].as_str();
            let data = fs::read_to_string(format!("data/{}.txt", day)).unwrap();
            match day {
                "day1" => {
                    day1::solve1(&data);
                    day1::solve2(&data);
                    day1::solve_aho_corasick(&data);
                }
                "day2" => {
                    day2::solve1(&data);
                    day2::solve2(&data);
                }
                _ => {
                    panic!("Invalid arg!");
                }
            }
        }
    }
}