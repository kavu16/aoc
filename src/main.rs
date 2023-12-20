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
                "day3" => {
                    day3::solve1(&data);
                    day3::solve2(&data);
                }
                "day4" => {
                    day4::solve1(&data);
                    day4::solve2(&data);
                }
                "day5" => {
                    day5::solve1(&data);
                    day5::solve2(&data);
                }
                "day6" => {
                    day6::solve1(&data);
                    day6::solve2(&data);
                }
                "day7" => {
                    day7::solve1(&data);
                    day7::solve2(&data);
                }
                "day8" => {
                    day8::solve1(&data);
                    day8::solve2(&data);
                }
                "day9" => {
                    day9::solve1(&data);
                    day9::solve2(&data);
                }
                "day10" => {
                    day10::solve1(&data);
                    day10::solve2(&data);
                }
                "day11" => {
                    day11::solve1(&data);
                    day11::solve2(&data);
                }
                "day12" => {
                    day12::solve1(&data);
                    day12::solve2(&data);
                }
                "day13" => {
                    day13::solve1(&data);
                    day13::solve2(&data);
                }
                _ => {
                    panic!("Invalid arg!");
                }
            }
        }
    }
}