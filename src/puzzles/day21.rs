use std::collections::{VecDeque, HashSet};

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

    assert_eq!(solve2(&example1), 16);
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
    let res = garden.iter()
                           .flatten()
                           .filter(|(plot, _)| plot == &'O')
                           .count();

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let mut garden = Vec::<Vec<char>>::new();
    for line in data.lines() {
        garden.push(
            line.chars()
                .collect::<Vec<char>>()
        );
    }

    let row = garden.len();
    let col = garden[0].len();
    assert_eq!(row, col);
    let size = row;

    let total_steps = 26_501_365;
    let original = total_steps % (2 * size);

    let reachable_plots = |x: usize| -> usize {
        let mut plots = HashSet::<(i128, i128)>::new();
        let mut visited = HashSet::<(i128, i128)>::new();
        let mut queue = VecDeque::<(i128, i128, usize)>::new();

        let start = ((row/2) as i128, (col/2) as i128, original + 2 * size * x);
        println!("{start:?}");
        queue.push_front(start); 
        visited.insert((start.0, start.1));

        while !queue.is_empty() {
            let (r, c, step) = queue.pop_front().unwrap();

            if step % 2 == 0 {
                plots.insert((r, c));
            }
            if step == 0 {
                continue;
            }
            
            for (nr, nc) in [(r+1, c), (r-1, c), (r, c+1), (r, c-1)] {
                if garden[nr.rem_euclid(size as i128) as usize][nc.rem_euclid(size as i128) as usize] == '#' || visited.contains(&(nr, nc)) {
                    continue;
                }
                queue.push_back((nr, nc, step - 1));
                visited.insert((nr, nc));
            }
        }
        plots.len()
    };

    
    // as you expand, assuming the rate in each direction stays the same, the number of visited plots will approach a 
    // quadratic form.  You simply need to interpolate the quadratic function and then insert the desired steps
    // The following finds where you start to converge (equal second differences) and then interpolates

    let mut x = 0;
    let mut vals = VecDeque::<usize>::new();
    let mut fdiff: [usize; 3];
    let mut sdiff: [usize; 2];

    loop {
        vals.push_back(reachable_plots(x));
        x += 1;
        println!("vals, x = {vals:?}, {x}");
        if vals.len() >= 4 {
            fdiff = [vals[1]-vals[0], vals[2]-vals[1], vals[3]-vals[2]];
            sdiff = [fdiff[2]-fdiff[1], fdiff[1]-fdiff[0]];
            if sdiff[1] == sdiff[0] {
                break;
            } else {
                vals.pop_front();
            }
        }
    }

    let offset = x - 4;

    let alpha = vals[0] as i128;
    let beta = vals[1] as i128;
    let gamma = vals[2] as i128;

    let c = alpha;
    let a = (gamma - 2 * beta + c) / 2;
    let b = beta - c - a;

    let quad = |x: i128| {
        (a * x.pow(2) + b * x + c) as usize
    };
    
    // The quadratic function is dependent on the number of expansions (not total steps) (imagine the case where
    // you just walk in a straight line).  Therefore, we have to calculate our final answer based off of the expansions
    // we have - an offset based on how we found the interpolation
    
    let res = quad(total_steps as i128 / (2 * size as i128) - offset as i128);
    println!("Part 2 = {}", res);
    res
}
