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

    fn galaxy_taxi_dist(x: (i32, i32), y: (i32, i32), empty_x: &HashSet<usize>, empty_y: &HashSet<usize>) -> i32 {
        let x_min = std::cmp::min(x.0, y.0);
        let x_max = std::cmp::max(x.0, y.0);
        let y_min = std::cmp::min(x.1, y.1);
        let y_max = std::cmp::max(x.1, y.1);
        let mut expansion = 0;
        for x in x_min..x_max {
            if empty_x.contains(&(x as usize)) {
                expansion += 1;
            }
        }
        for y in y_min..y_max {
            if empty_y.contains(&(y as usize)) {
                expansion += 1;
            }
        }
        (x_max - x_min) + (y_max - y_min) + expansion
    }

    let mut res = 0;

    galaxy_locs.iter().for_each(|g| {
        res += galaxy_locs.iter().map(|d| galaxy_taxi_dist(*g, *d, &empty_rows, &empty_cols)).sum::<i32>();
    });

    res /= 2;

    println!("Part 1 = {}", res);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let galaxy: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect::<Vec<char>>()).collect();
    let rows = galaxy.len();
    let cols = galaxy[0].len();

    let mut empty_rows: HashSet<usize> = (0..rows).into_iter().collect();
    let mut empty_cols: HashSet<usize> = (0..cols).into_iter().collect();
    let mut galaxy_locs: Vec<(i64, i64)> = vec![];

    for r in 0..rows {
        for c in 0..cols {
            if galaxy[r][c] == '#' {
                empty_rows.remove(&r);
                empty_cols.remove(&c);
                galaxy_locs.push((r as i64, c as i64));
            }
        }
    }

    fn galaxy_taxi_dist(x: (i64, i64), y: (i64, i64), empty_x: &HashSet<usize>, empty_y: &HashSet<usize>) -> i64 {
        let x_min = std::cmp::min(x.0, y.0);
        let x_max = std::cmp::max(x.0, y.0);
        let y_min = std::cmp::min(x.1, y.1);
        let y_max = std::cmp::max(x.1, y.1);
        let mut expansion = 0;
        for x in x_min..x_max {
            if empty_x.contains(&(x as usize)) {
                expansion += 999_999;
            }
        }
        for y in y_min..y_max {
            if empty_y.contains(&(y as usize)) {
                expansion += 999_999;
            }
        }
        (x_max - x_min) + (y_max - y_min) + expansion
    }

    let mut res = 0;

    galaxy_locs.iter().for_each(|g| {
        res += galaxy_locs.iter().map(|d| galaxy_taxi_dist(*g, *d, &empty_rows, &empty_cols)).sum::<i64>();
    });

    res /= 2;

    println!("Part 2 = {}", res);
}

//  [2 5 3 4 8]
//  [0 2 7 10 14]
//  [20 15 12 8 0]
//  [2 27 19 18 14]