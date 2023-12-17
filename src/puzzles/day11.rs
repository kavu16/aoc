use std::collections::HashSet;

#[test]

fn test() {
    let data = String::from(
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    );

    solve1(&data);

    solve2(&data);
}

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let galaxy: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();
    let rows = galaxy.len();
    let cols = galaxy[0].len();

    let mut empty_rows: HashSet<usize> = (0..rows).into_iter().collect();
    let mut empty_cols: HashSet<usize> = (0..cols).into_iter().collect();
    let mut galaxy_locs: Vec<(i32, i32)> = vec![];

    for r in 0..rows {
        for c in 0..cols {
            if galaxy[r][c] == '#' {
                empty_rows.remove(&r);
                empty_cols.remove(&c);
                galaxy_locs.push((r as i32, c as i32));
            }
        }
    }
    
    println!("Number of galaxies = {}", galaxy_locs.len());
    println!("Part 1 = {}", 0);
}

pub fn solve2(data: &String) {
    let _data = data.lines().collect::<Vec<_>>();

    println!("Part 2 = {}", 0);
}

//  [2 5 3 4 8]
//  [0 2 7 10 14]
//  [20 15 12 8 0]
//  [2 27 19 18 14]