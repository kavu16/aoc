use std::collections::HashMap;

#[test]

fn test() {
    let data = String::from(
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    );

    assert_eq!(solve1(&data), 136);

    assert_eq!(solve2(&data), 64);
}

pub fn solve1(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut platform: Vec<Vec<char>> = Vec::new();

    for line in data {
        platform.push(line.chars().collect::<Vec<char>>());
    }
    let height = platform.len();
    let length = platform[0].len();

    let mut res = 0;
    for l in 0..length {
        let mut score = 0;
        let mut curr_value = height;
        let mut rock_count = 0;
        for h in 0..height {
            if platform[h][l] == '#' {
                for _rock in 0..rock_count {
                    score += curr_value;
                    curr_value -= 1;
                }
                curr_value = height - h - 1;
                rock_count = 0;
            } else if platform[h][l] == 'O' {
                rock_count += 1;
            }
        }
        for _rock in 0..rock_count {
            score += curr_value;
            curr_value -= 1;
        }
        
        res += score;
    }

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut platform: Vec<Vec<char>> = Vec::new();

    for line in data {
        platform.push(line.chars().collect::<Vec<char>>());
    }
    let height = platform.len();

    let mut res = 0;
    let mut cycle_map = HashMap::new();
    let mut c_mod = 0;
    let mut c_offset = 0;

    for c in 0..1_000_000_000 {
        cycle(&mut platform);

        if let Some(old) = cycle_map.insert(platform.clone(), c) {
            c_offset = old;
            c_mod = c - old;
            break;
        }
    }

    let repeat = (1_000_000_000 - c_offset) % c_mod - 1;

    for _ in 0..repeat {
        cycle(&mut platform);
    }

    for (index, row) in platform.iter().enumerate() {
        for c in row {
            if c == &'O' {
                res += height - index;
            }
        }
    }

    println!("Part 2 = {}", res);
    res
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    let height = platform.len();
    let length = platform[0].len();
    //North
    for l in 0..length {
        let mut curr_value = height;
        let mut rock_count = 0;
        for h in 0..height {
            if platform[h][l] == '#' {
                for _rock in 0..rock_count {
                    platform[height - curr_value][l] = 'O';
                    curr_value -= 1;
                }
                curr_value = height - h - 1;
                rock_count = 0;
            } else if platform[h][l] == 'O' {
                rock_count += 1;
                platform[h][l] = '.';
            }
        }
        for _rock in 0..rock_count {
            platform[height - curr_value][l] = 'O';
            curr_value -= 1;
        }
    }

    //West
    for h in 0..height {
        let mut curr_value = length;
        let mut rock_count = 0;
        for l in 0..length {
            if platform[h][l] == '#' {
                for _rock in 0..rock_count {
                    platform[h][length - curr_value] = 'O';
                    curr_value -= 1;
                }
                curr_value = length - l - 1;
                rock_count = 0;
            } else if platform[h][l] == 'O' {
                rock_count += 1;
                platform[h][l] = '.';
            }
        }
        for _rock in 0..rock_count {
            platform[h][length - curr_value] = 'O';
            curr_value -= 1;
        }
    }

    //South
    for l in 0..length {
        let mut curr_value = height;
        let mut rock_count = 0;
        for h in (0..height).rev() {
            if platform[h][l] == '#' {
                for _rock in 0..rock_count {
                    platform[curr_value-1][l] = 'O';
                    curr_value -= 1;
                }
                curr_value = h;
                rock_count = 0;
            } else if platform[h][l] == 'O' {
                rock_count += 1;
                platform[h][l] = '.';
            }
        }
        for _rock in 0..rock_count {
            platform[curr_value-1][l] = 'O';
            curr_value -= 1;
        } 
    }

    //East
    for h in 0..height {
        let mut curr_value = length;
        let mut rock_count = 0;
        for l in (0..length).rev() {
            if platform[h][l] == '#' {
                for _rock in 0..rock_count {
                    platform[h][curr_value-1] = 'O';
                    curr_value -= 1;
                }
                curr_value = l;
                rock_count = 0;
            } else if platform[h][l] == 'O' {
                rock_count += 1;
                platform[h][l] = '.';
            }
        }
        for _rock in 0..rock_count {
            platform[h][curr_value-1] = 'O';
            curr_value -= 1;
        } 
    }
}