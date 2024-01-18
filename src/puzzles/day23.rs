// use std::collections::{BinaryHeap, HashSet};

#[test]

fn test() {
    let example1 = String::from(
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
    );

    println!("Example 1:");
    assert_eq!(solve1(&example1), 94);
    assert_eq!(solve2(&example1), 0);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    None,
    North,
    South, 
    East, 
    West,
}
use Direction::*;



pub fn solve1(data: &String) -> usize {
    let mut grid = Vec::<Vec<char>>::new();
    for line in data.lines() {
        grid.push(line.chars().collect());
    }
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    fn hike(r: i32, c: i32, last: Direction, grid: &Vec<Vec<char>>, rows: i32, cols: i32) -> usize {
        if (r, c) == (rows-1, cols-2) {
            return 1;
        }
        if r < 0 || r >= rows || c < 0 || c >= cols || grid[r as usize][c as usize] == '#' {
            return 0;
        }
        if (grid[r as usize][c as usize] == 'v' && last != South) ||
            (grid[r as usize][c as usize] == '^' && last != North) ||
            (grid[r as usize][c as usize] == '<' && last != West) ||
            (grid[r as usize][c as usize] == '>' && last != East) 
        {
            return 0;
        }
        let n = if last != South {
            hike(r-1, c, North, grid, rows, cols)
        } else {
            0
        };
        let s = if last != North {
            hike(r+1,c, South, grid, rows, cols)
        } else {
            0
        };
        let e = if last != West {
            hike(r, c+1, East, grid, rows, cols)
        } else {
            0
        };
        let w = if last != East {
            hike(r, c-1, West, grid, rows, cols)
        } else {
            0
        };
        1 + [n,e,s,w].iter().max().unwrap()
    }

    let res = hike(0,1,None, &grid, rows, cols) - 1;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(_data: &String) -> usize {
    let res = 0;
    println!("Part 2 = {}", res);
    res
}