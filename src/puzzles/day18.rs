#[test]

fn test() {
    let data = String::from(
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
    );

    assert_eq!(solve1(&data), 62);

    assert_eq!(solve2(&data), 952408144115);
}

pub fn solve1(data: &String) -> i32 {
    let data = data.lines().collect::<Vec<_>>();
    let mut coords = Vec::<(i32, i32)>::new();
    let (mut r, mut c) = (0, 0);
    coords.push((r,c));

    let mut count = 0;
    for line in data {
        let (dir, tail) = line.split_once(' ').unwrap();
        let (steps, _color) = tail.split_once(' ').unwrap();
        let steps = steps.parse::<i32>().unwrap();

        count += steps;
        match dir {
            "R" => {
                c += steps;
                coords.push((r,c));
            }
            "L" => {
                c -= steps;
                coords.push((r,c));
            }
            "U" => {
                r -= steps;
                coords.push((r,c));
            }
            "D" => {
                r += steps;
                coords.push((r,c));
            }
            _ => println!("Invalid dig instructions!"),
        }
    }

    // Used Shoestring Theorem and Pick's theorem.
    // The answer is equivalent to the interior points of the coords traced out by the instructions
    let mut res = 0;
    for i in (1..coords.len()).rev() {
        res += coords[i].0 * coords[i-1].1 - coords[i-1].0 * coords[i].1;
    }
    res /= 2;
    res = res + count/2 + 1;

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> i128 {
    let data = data.lines().collect::<Vec<_>>();
    let mut coords = Vec::<(i128, i128)>::new();
    let (mut r, mut c) = (0, 0);
    coords.push((r,c));

    let mut count = 0;
    for line in data {
        let (_dir, tail) = line.split_once(' ').unwrap();
        let (_steps, color) = tail.split_once(' ').unwrap();
        let color = color.trim_start_matches("(#").trim_end_matches(")");
        let mut steps = 0;
        let mut dir: char = '0';
        for (i, n) in color.chars().enumerate() {
            if i == 5 {
                dir = n;
                continue;
            }
            steps = 16*steps + n.to_digit(16).unwrap() as i128;
        }
        count += steps;
        match dir {
            '0' => {
                c += steps;
                coords.push((r,c));
            }
            '2' => {
                c -= steps;
                coords.push((r,c));
            }
            '3' => {
                r -= steps;
                coords.push((r,c));
            }
            '1' => {
                r += steps;
                coords.push((r,c));
            }
            _ => println!("Invalid dig instructions!"),
        }
    }

    // Used Shoestring Theorem and Pick's theorem.
    // The answer is equivalent to the interior points of the coords traced out by the instructions
    let mut res = 0;
    for i in (1..coords.len()).rev() {
        res += coords[i].0 * coords[i-1].1 - coords[i-1].0 * coords[i].1;
    }
    res /= 2;
    res = res + count/2 + 1;

    println!("Part 2 = {}", res);
    res
}
