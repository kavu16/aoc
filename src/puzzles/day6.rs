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

    let times: Vec<i32> = data[0].split_whitespace()[1..].iter().map(|t| t.parse::<i32>()).collect();
    let distances: Vec<i32> = data[1].split_whitespace()[1..].iter().map(|t| t.parse::<i32>()).collect();

    let races = times.zip(&distances);

    for (t, d) in races {
        println!("Time = {t}, distance = {d}");
    }

    println!("Part 1 = {}", 0);
}

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();

    println!("Part 2 = {}", 0);
}