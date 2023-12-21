use std::collections::{HashSet, HashMap};

#[test]

fn test() {
    let data = String::from(
r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
    );

    assert_eq!(solve1(&data), 46);

    assert_eq!(solve2(&data), 51);
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn dfs(d: Direction, r: usize, c: usize, grid: &Vec<Vec<char>>, visited: &mut HashMap<(usize, usize), HashSet<Direction>>) {
    if let Some(beams) = visited.get_mut(&(r, c)) {
        if beams.contains(&d) {
            return;
        } else {
            beams.insert(d);
        }
    } else {
        visited.insert((r,c), HashSet::from([d]));
    }
    
    let rows = grid.len()-1;
    let cols = grid[0].len()-1;

    match grid[r][c] {
        '.' => {
            match d {
                Direction::Right => {
                    if c == cols {
                        return;
                    } else {
                        dfs(Direction::Right, r, c+1, grid, visited);
                    }
                }
                Direction::Left => {
                    if c == 0 {
                        return;
                    } else {
                        dfs(Direction::Left, r, c-1, grid, visited);
                    }
                }
                Direction::Down => {
                    if r == rows {
                        return;
                    } else {
                        dfs(Direction::Down, r+1, c, grid, visited);
                    }
                }
                Direction::Up => {
                    if r == 0 {
                        return;
                    } else {
                        dfs(Direction::Up, r-1, c, grid, visited);
                    }
                }
            }
        }
        '/' => {
            match d {
                Direction::Right => {
                    if r == 0 {
                        return;
                    } else {
                        dfs(Direction::Up, r-1, c, grid, visited);
                    }
                }
                Direction::Left => {
                    if r == rows {
                        return;
                    } else {
                        dfs(Direction::Down, r+1, c, grid, visited);
                    }
                }
                Direction::Down => {
                    if c == 0 {
                        return;
                    } else {
                        dfs(Direction::Left, r, c-1, grid, visited);
                    }
                }
                Direction::Up => {
                    if c == cols {
                        return;
                    } else {
                        dfs(Direction::Right, r, c+1, grid, visited);
                    }
                }
            }
        }
        '\\' => {
            match d {
                Direction::Right => {
                    if r == rows {
                        return;
                    } else {
                        dfs(Direction::Down, r+1, c, grid, visited);
                    }
                }
                Direction::Left => {
                    if r == 0 {
                        return;
                    } else {
                        dfs(Direction::Up, r-1, c, grid, visited);
                    }
                }
                Direction::Down => {
                    if c == cols {
                        return;
                    } else {
                        dfs(Direction::Right, r, c+1, grid, visited);
                    }
                }
                Direction::Up => {
                    if c == 0 {
                        return;
                    } else {
                        dfs(Direction::Left, r, c-1, grid, visited);
                    }
                }
            }
        }
        '|' => {
            match d {
                Direction::Right | Direction::Left => {
                    if r == 0 {
                        dfs(Direction::Down, r+1, c, grid, visited);
                    } else if r == rows {
                        dfs(Direction::Up, r-1, c, grid, visited);
                    } else {
                        dfs(Direction::Down, r+1, c, grid, visited);
                        dfs(Direction::Up, r-1, c, grid, visited);
                    }
                }
                Direction::Down => {
                    if r == rows {
                        return;
                    } else {
                        dfs(Direction::Down, r+1, c, grid, visited);
                    }
                }
                Direction::Up => {
                    if r == 0 {
                        return;
                    } else {
                        dfs(Direction::Up, r-1, c, grid, visited);
                    }
                }
            }
        }
        '-' => {
            match d {
                Direction::Right => {
                    if c == cols {
                        return;
                    } else {
                        dfs(Direction::Right, r, c+1, grid, visited);
                    }
                }
                Direction::Left => {
                    if c == 0 {
                        return;
                    } else {
                        dfs(Direction::Left, r, c-1, grid, visited);
                    }
                }
                Direction::Down | Direction::Up => {
                    if c == 0 {
                        dfs(Direction::Right, r, c+1, grid, visited);
                    } else if c == cols {
                        dfs(Direction::Left, r, c-1, grid, visited);
                    } else {
                        dfs(Direction::Right, r, c+1, grid, visited);
                        dfs(Direction::Left, r, c-1, grid, visited);
                    }
                }
            }
        }
        _ => return,
    }
}

pub fn solve1(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut visited = HashMap::new();
    dfs(Direction::Right, 0, 0, &grid, &mut visited);
    let res = visited.len();

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let rows = grid.len();
    let cols = grid[0].len();

    let mut energized = 0;
    for r in 0..rows {
        let mut visited = HashMap::new();
        dfs(Direction::Right, r, 0, &grid, &mut visited);
        energized = std::cmp::max(energized, visited.len());
        visited.clear();
        dfs(Direction::Left, r, cols-1, &grid, &mut visited);
        energized = std::cmp::max(energized, visited.len());
    }
    for c in 0..cols {
        let mut visited = HashMap::new();
        dfs(Direction::Down, 0, c, &grid, &mut visited);
        energized = std::cmp::max(energized, visited.len());
        visited.clear();
        dfs(Direction::Up, rows-1, c, &grid, &mut visited);
        energized = std::cmp::max(energized, visited.len());
    }
    let res = energized;
    println!("Part 2 = {}", res);
    res
}
