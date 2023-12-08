#[test]

fn test() {
    let data = String::from(
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    );

    solve1(&data);

    solve2(&data);
}

// use::std::collections::HashMap;

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();

    let mut seeds: Vec<(u64, bool)> = data[0]
                                .split_whitespace()
                                .filter(|s| s.parse::<u64>().is_ok())
                                .map(|s| (s.parse::<u64>().unwrap(), false))
                                .collect();

    for line in &data[2..] {
        if !line.is_empty() && !line.contains("map") {
            let dsr: Vec<u64> = line.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
            let dest = dsr[0];
            let src = dsr[1];
            let rng = dsr[2];

            seeds = seeds.iter().map(|(seed, seen)| {
                if !seen && src <= *seed && *seed <= src + rng {
                    let idx = *seed - src;
                    ((dest + idx), true)
                } else {
                    (*seed, *seen)
                }
            }).collect();
        } else {
            seeds = seeds.iter().map(|(seed, _)| (*seed, false)).collect();
        }
    }

    let loc = seeds.iter().map(|(seed, _)| *seed).min().unwrap();

    println!("Part 1 = {}", loc);
}

pub fn solve2(data: &String) {
    let mut loc = 0;

    println!("Part 2 = {}", loc);
}