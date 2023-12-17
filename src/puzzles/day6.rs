#[test]

fn test() {
    let data = String::from(
"Time:      7  15   30
Distance:  9  40  200"
    );

    solve1(&data);

    solve2(&data);
}

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();

    let times = data[0].split_whitespace().filter(|t| t.parse::<i32>().is_ok()).map(|t| t.parse::<i32>().unwrap());
    let distances = data[1].split_whitespace().filter(|d| d.parse::<i32>().is_ok()).map(|d| d.parse::<i32>().unwrap());

    // let races: Vec<(i32, i32)> = times.zip(distances).collect();
    let mut res = 1;

    for (t, d) in times.zip(distances) {
        res *= (0..t+1).filter(|r| r*(t-r) > d).count();
    }

    println!("Part 1 = {}", res);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();

    let (_, t) = data[0].split_once(' ').unwrap();
    let (_, d) = data[1].split_once(' ').unwrap();

    let time: u64 = t.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap();
    let distance: u64 = d.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap();

    let res = (0..time).filter(|r| r*(time-r) > distance).count();

    println!("Part 2 = {}", res);
}