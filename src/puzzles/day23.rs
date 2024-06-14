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
    assert_eq!(solve2(&example1), 154);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    None,
    North,
    South, 
    East, 
    West,
}
use std::collections::{HashMap, HashSet, VecDeque};

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

pub fn solve2(data: &String) -> usize {
    let mut grid = Vec::<Vec<char>>::new();
    for line in data.lines() {
        grid.push(line.chars().collect());
    }
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut graph = HashMap::<(i32,i32), Vec<((i32,i32),i32)>>::new();
    let start = (0,1);
    let end = (rows-1, cols-2);
    graph.insert(start, Vec::new());
    graph.insert(end, Vec::new());

    for r in 0..rows {
        for c in 0..cols {
            if grid[r as usize][c as usize] == '#' {continue;}

            let mut paths = 0;
            for [dr, dc] in [[0,1], [0,-1], [1,0], [-1,0]] {
                if r + dr < 0 || r + dr == rows || 
                    c + dc < 0 || c + dc == cols || 
                    grid[(r+dr) as usize][(c+dc) as usize] == '#'
                 {
                    continue;
                }
                paths += 1;
            }
            if paths > 2 { graph.insert((r,c), Vec::new()); }
        }
    }

    let mut queue = VecDeque::<((i32,i32),(i32,i32),i32)>::from(graph
        .keys()
        .map(|&(r, c)| ((r, c), (r, c), 0i32))
        .collect::<Vec<((i32,i32),(i32, i32),i32)>>()
    );

    let mut seen = HashSet::<((i32,i32), (i32,i32))>::new();
    while let Some((parent, curr, steps)) = queue.pop_front() {
        if let Some(node) = graph.get_mut(&curr) {
            if curr != parent {
                node.push((parent, steps));
                continue;
            }
        }
        seen.insert((parent, curr));
        let (r,c) = curr;
        for [dr, dc] in [[0,1], [0,-1], [1,0], [-1,0]] {
            if r + dr < 0 || r + dr == rows ||
                c + dc < 0 || c + dc == cols ||
                seen.contains(&(parent, (r+dr, c+dc))) ||
                grid[(r+dr) as usize][(c+dc) as usize] == '#' {
                    continue;
                }
            
            queue.push_back((parent, (r+dr, c+dc), steps + 1));
        }
    }

    let mut queue = VecDeque::<(i32,i32)>::new();
    let mut seen = HashSet::<(i32,i32)>::new();
    let mut border = Vec::<((i32,i32), (i32,i32))>::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if seen.contains(&node) {continue;}
        seen.insert(node);
        for (neighbor, _steps) in graph.get(&node).unwrap() {
            if let Some(adj) = graph.get(neighbor) {
                if adj.len() == 3 && !seen.contains(neighbor) {
                    border.push((node, *neighbor));
                    queue.push_back(*neighbor);
                }
            }
        }
    }

    border.iter().for_each(|(parent, child)| {
        if let Some(kill_idx) = graph.get(child).unwrap().iter().position(|&i| i.0 == *parent) {
            graph.get_mut(child).unwrap().remove(kill_idx);
        }
    });

    fn hike(curr_node: (i32, i32),
            curr_steps: i32,
            graph: &HashMap<(i32, i32), Vec<((i32,i32), i32)>>, 
            visited: &mut HashSet<(i32,i32)>,
            end: (i32,i32)
        ) -> i32 {
        if curr_node == end { return curr_steps; }
        if visited.contains(&curr_node) { return 0; }

        visited.insert(curr_node);
        let mut max_hike = curr_steps;
        for (node, steps) in graph.get(&curr_node).unwrap() {
            max_hike = max_hike.max(hike(*node, curr_steps + steps, graph, visited, end));
        }
        visited.remove(&curr_node);

        max_hike
    }

    let mut visited = HashSet::<(i32,i32)>::new();
    let res = hike(start, 0, &graph, &mut visited, end) as usize;
    println!("Part 2 = {}", res);
    res
}