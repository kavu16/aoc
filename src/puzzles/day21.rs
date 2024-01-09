use std::collections::{HashMap, VecDeque};

#[test]

fn test() {
    let example1 = String::from(
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
    );
    // S at (5, 5)
    // assert_eq!(solve1(&example1), 16);

    assert_eq!(solve2(&example1), 50);
}

pub fn solve1(data: &String) -> usize {
    let mut garden = Vec::<Vec<(char,char)>>::new();
    for line in data.lines() {
        garden.push(line.chars().map(|c| (c, c)).collect::<Vec<(char, char)>>());
    }
    let (row, col) = (garden.len(), garden[0].len());
    let start = (65, 65);
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(start);

    for _ in 0..64 {
        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().unwrap();
            if r > 0 && garden[r-1][c] != ('#', '#') && !queue.contains(&(r-1, c)) {
                garden[r-1][c].1 = 'O';
                queue.push_back((r-1, c));
            }
            if r < row - 1 && garden[r+1][c] != ('#', '#') && !queue.contains(&(r+1, c)) {
                garden[r+1][c].1 = 'O';
                queue.push_back((r+1, c));
            }
            if c > 0 && garden[r][c-1] != ('#', '#') && !queue.contains(&(r, c-1)) {
                garden[r][c-1].1 = 'O';
                queue.push_back((r, c-1));
            }
            if c < col - 1 && garden[r][c+1] != ('#', '#') && !queue.contains(&(r, c+1)) {
                garden[r][c+1].1 = 'O';
                queue.push_back((r, c+1));
            }
        }
        for r in 0..row {
            for c in 0..col {
                let (prev, new) = garden[r][c];
                garden[r][c] = (new, prev);
            }
        }
    }
    let res = garden.iter().flatten().filter(|(plot, _)| plot == &'O').count();

    // let res = 0;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let mut garden = Vec::<Vec<((char, usize),(char, usize))>>::new();
    for line in data.lines() {
        garden.push(line.chars().map(|c| ((c,0), (c,0))).collect::<Vec<((char, usize),(char, usize))>>());
    }
    let (row, col) = (garden.len(), garden[0].len());
    let start = (5, 5);
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(start);

    for _ in 0..50 {
        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().unwrap();
            if garden[(row + r-1) % row][c] != (('#', 0), ('#', 0)) && !queue.contains(&((row + r-1) % row, c)) {
                garden[(row + r-1) % row][c].1.0 = 'O';
                garden[(row + r-1) % row][c].1.1 += 1;
                queue.push_back(((row + r-1) % row, c));
            }
            if garden[(row + r+1) % row][c] != (('#', 0), ('#', 0)) && !queue.contains(&((row + r+1) % row, c)) {
                garden[(row + r+1) % row][c].1.0 = 'O';
                garden[(row + r+1) % row][c].1.1 += 1;
                queue.push_back(((row + r+1) % row, c));
            }
            if garden[r][(col + c-1) % col] != (('#', 0), ('#', 0)) && !queue.contains(&(r, (col + c-1) % col)) {
                garden[r][(col + c-1) % col].1.0 = 'O';
                garden[r][(col + c-1) % col].1.1 += 1;
                queue.push_back((r, (col + c-1) % col));
            }
            if garden[r][(col + c+1) % col] != (('#', 0), ('#', 0)) && !queue.contains(&(r, (col + c+1) % col)) {
                garden[r][(col + c+1) % col].1.0 = 'O';
                garden[r][(col + c+1) % col].1.1 += 1;
                queue.push_back((r, (col + c+1) % col));
            }
        }
        for r in 0..row {
            for c in 0..col {
                let (prev, new) = garden[r][c];
                garden[r][c] = (new, prev);
            }
        }
    }
    let res = garden.iter().flatten().filter_map(|((plot, count), _)| if plot == &'O' {Some(count)} else {None}).sum();
    println!("Part 2 = {}", res);
    res
}
