use std::{collections::{BinaryHeap, HashSet}, cmp::Ordering};


#[test]

fn test() {
    let data = String::from(
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    );

    assert_eq!(solve1(&data), 102);

    assert_eq!(solve2(&data), 94);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left, 
    Up,
    Down,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Right => Down,
            Left => Up,
            Up => Right,
            Down => Left,
        }
    }

    fn left(&self) -> Self {
        match self {
            Right => Up,
            Left => Down,
            Up => Left,
            Down => Right,
        }
    }
}
use Direction::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Node {
    dist: usize,
    pos: (usize, usize),
    dir: Option<Direction>,
    traveled: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<const MIN: usize, const MAX: usize>(graph: &mut Vec<Vec<usize>>) -> usize {
    let mut pq = BinaryHeap::<Node>::new();
    let mut visited = HashSet::<((usize, usize), Option<Direction>, usize)>::new();

    let start: Node = Node { dist: 0, pos: (0,0), dir: Some(Right), traveled: 0 };
    visited.insert((start.pos, start.dir, start.traveled));
    pq.push(start);

    while let Some(Node {
        dist,
        pos,
        dir: Some(d),
        traveled,
    }) = pq.pop() {
        let (r, c) = pos;
        if (r, c) == (graph.len()-1, graph[0].len()-1) && traveled >= MIN {
            return dist;
        }

        if traveled >= MIN {
            // Turn Right
            match d {
                Right if r < graph.len()-1 => {
                    let new_right: Node = Node { dist: dist + graph[r+1][c], pos: (r+1, c), dir: Some(d.right()), traveled: 1 };
                    if visited.insert((new_right.pos, new_right.dir, new_right.traveled)) {
                        pq.push(new_right);
                    }
                }
                Left if r > 0 => {
                    let new_right: Node = Node { dist: dist + graph[r-1][c], pos: (r-1, c), dir: Some(d.right()), traveled: 1 };
                    if visited.insert((new_right.pos, new_right.dir, new_right.traveled)) {
                        pq.push(new_right);
                    }
                }
                Up if c < graph[0].len() - 1 => {
                    let new_right: Node = Node { dist: dist + graph[r][c+1], pos: (r, c+1), dir: Some(d.right()), traveled: 1 };
                    if visited.insert((new_right.pos, new_right.dir, new_right.traveled)) {
                        pq.push(new_right);
                    }
                }
                Down if c > 0 => {
                    let new_right: Node = Node { dist: dist + graph[r][c-1], pos: (r, c-1), dir: Some(d.right()), traveled: 1 };
                    if visited.insert((new_right.pos, new_right.dir, new_right.traveled)) {
                        pq.push(new_right);
                    }
                }
                _ => (),
            }
            // Turn Left
            match d {
                Right if r > 0 => {
                    let new_left: Node = Node { dist: dist + graph[r-1][c], pos: (r-1, c), dir: Some(d.left()), traveled: 1 };
                    if visited.insert((new_left.pos, new_left.dir, new_left.traveled)) {
                        pq.push(new_left);
                    }
                }
                Left if r < graph.len() - 1 => {
                    let new_left: Node = Node { dist: dist + graph[r+1][c], pos: (r+1, c), dir: Some(d.left()), traveled: 1 };
                    if visited.insert((new_left.pos, new_left.dir, new_left.traveled)) {
                        pq.push(new_left);
                    }
                }
                Up if c > 0 => {
                    let new_left: Node = Node { dist: dist + graph[r][c-1], pos: (r, c-1), dir: Some(d.left()), traveled: 1 };
                    if visited.insert((new_left.pos, new_left.dir, new_left.traveled)) {
                        pq.push(new_left);
                    }
                }
                Down if c < graph[0].len() - 1 => {
                    let new_left: Node = Node { dist: dist + graph[r][c+1], pos: (r, c+1), dir: Some(d.left()), traveled: 1 };
                    if visited.insert((new_left.pos, new_left.dir, new_left.traveled)) {
                        pq.push(new_left);
                    }
                }
                _ => (),
            }
        }

        // Go Straight
        if traveled < MAX {
            match d {
                Right if c < graph[0].len() - 1 => {
                    let new_straight: Node = Node { dist: dist + graph[r][c+1], pos: (r, c+1), dir: Some(d), traveled: traveled + 1 };
                    if visited.insert((new_straight.pos, new_straight.dir, new_straight.traveled)) {
                        pq.push(new_straight);
                    }
                }
                Left if c > 0 => {
                    let new_straight: Node = Node { dist: dist + graph[r][c-1], pos: (r, c-1), dir: Some(d), traveled: traveled + 1 };
                    if visited.insert((new_straight.pos, new_straight.dir, new_straight.traveled)) {
                        pq.push(new_straight);
                    }
                }
                Up if r > 0 => {
                    let new_straight: Node = Node { dist: dist + graph[r-1][c], pos: (r-1, c), dir: Some(d), traveled: traveled + 1 };
                    if visited.insert((new_straight.pos, new_straight.dir, new_straight.traveled)) {
                        pq.push(new_straight);
                    }
                }
                Down if r < graph.len() - 1 => {
                    let new_straight: Node = Node { dist: dist + graph[r+1][c], pos: (r+1, c), dir: Some(d), traveled: traveled + 1 };
                    if visited.insert((new_straight.pos, new_straight.dir, new_straight.traveled)) {
                        pq.push(new_straight);
                    }
                }
                _ => (),
            }
        }
    }
    0
}

pub fn solve1(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut graph: Vec<Vec<usize>> = Vec::new();
    for line in data {
        graph.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
    }

    let res = dijkstra::<0,3>(&mut graph);
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut graph: Vec<Vec<usize>> = Vec::new();
    for line in data {
        graph.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
    }

    let res = dijkstra::<4,10>(&mut graph);
    println!("Part 2 = {}", res);
    res
}
