#[test]

fn test() {
    solve1(&String::from(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    ));

    solve2(&String::from(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
    ));
}

pub fn test_neighbor(r: i32, c: i32, grid: &Vec<Vec<char>>) -> bool {
    if r < 0 || c < 0 || r as usize == grid.len() 
        || c as usize == grid.len() 
        || grid[r as usize][c as usize].is_numeric() 
        || grid[r as usize][c as usize] == '.' {
        false
    } else {
        true
    }
}

pub fn solve1(data: &String) {
    let grid = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut result = 0;

    let mut curr: u32 = 0;
    let mut engine_part_present = false;

    let row = grid.len();
    let col = grid[0].len();

    for r in 0..row {
        if engine_part_present {
            result += curr;
        }
        curr = 0;
        engine_part_present = false;
        for c in 0..col {
            if grid[r][c].is_numeric() {
                curr = 10 * curr + grid[r][c].to_digit(10).unwrap();
                engine_part_present = engine_part_present 
                                      || test_neighbor(r as i32 + 1, c as i32 + 1, &grid)
                                      || test_neighbor(r as i32 + 1, c as i32, &grid)
                                      || test_neighbor(r as i32 + 1, c as i32 - 1, &grid)
                                      || test_neighbor(r as i32, c as i32 + 1, &grid)
                                      || test_neighbor(r as i32, c as i32 - 1, &grid)
                                      || test_neighbor(r as i32 - 1, c as i32 + 1, &grid)
                                      || test_neighbor(r as i32 - 1, c as i32, &grid)
                                      || test_neighbor(r as i32 - 1, c as i32 - 1, &grid);
            } else {
                if engine_part_present {
                    result += curr;
                }
                curr = 0;
                engine_part_present = false;
            }
        }
    }

    println!("Part 1 = {}", result);
}

use::std::collections::{HashMap, HashSet};

pub fn gear_neighbor(r: i32, c: i32, grid: &Vec<Vec<char>>, curr_gears: &mut HashSet<(usize, usize)>) {
    if r < 0 || c < 0 || r as usize == grid.len() 
        || c as usize == grid.len() || grid[r as usize][c as usize] != '*' {
            return;
    } else {
        curr_gears.insert((r as usize, c as usize));
    }
}

pub fn solve2(data: &String) {
    let grid = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut result = 0;

    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    let mut curr: i32 = 0;
    let mut curr_gear = HashSet::new();

    let row = grid.len();
    let col = grid[0].len();

    for r in 0..row {
        if !curr_gear.is_empty() {
            for gear in curr_gear.iter() {
                if gears.contains_key(gear) {
                    gears.get_mut(gear).unwrap().push(curr);
                } else {
                    gears.insert(*gear, vec![curr]);
                }
            }
        }
        curr = 0;
        curr_gear.clear();

        for c in 0..col {
            if grid[r][c].is_numeric() {
                curr = 10 * curr + grid[r][c].to_digit(10).unwrap() as i32;
                gear_neighbor(r as i32 + 1, c as i32 + 1, &grid, &mut curr_gear);
                gear_neighbor(r as i32 + 1, c as i32, &grid, &mut curr_gear);
                gear_neighbor(r as i32 + 1, c as i32 - 1, &grid, &mut curr_gear);
                gear_neighbor(r as i32, c as i32 + 1, &grid, &mut curr_gear);
                gear_neighbor(r as i32, c as i32 - 1, &grid, &mut curr_gear);
                gear_neighbor(r as i32 - 1, c as i32 + 1, &grid, &mut curr_gear);
                gear_neighbor(r as i32 - 1, c as i32, &grid, &mut curr_gear);
                gear_neighbor(r as i32 - 1, c as i32 - 1, &grid, &mut curr_gear);
            } else {
                if !curr_gear.is_empty() {
                    for gear in curr_gear.iter() {
                        if gears.contains_key(gear) {
                            gears.get_mut(gear).unwrap().push(curr);
                        } else {
                            gears.insert(*gear, vec![curr]);
                        }
                    }
                }
                curr = 0;
                curr_gear.clear();
            }
        }
    }

    for (_, ratios) in gears {
        if ratios.len() == 2 {
            result += ratios[0] * ratios[1];
        }
    }
    println!("Part 2 = {result}");
}