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

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();

    let mut seeds: Vec<(u64, bool)> = data[0]
                                .split_whitespace()
                                .filter_map(|s| s.parse::<u64>().ok())
                                .map(|s| (s, false))
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
    let data = data.lines().collect::<Vec<_>>();

    let odd_seeds = data[0]
                                .split_whitespace()
                                .filter_map(|s| s.parse::<u64>().ok())
                                .enumerate()
                                .filter(|(index, _)| index % 2 == 1)
                                .map(|(_, s)| s);
    let even_seeds= data[0]
                                .split_whitespace()
                                .filter_map(|s| s.parse::<u64>().ok())
                                .enumerate()
                                .filter(|(index,_)| index % 2 == 0)
                                .map(|(_, s)| s);

    let seed_ranges: Vec<(u64,u64)> = even_seeds.zip(odd_seeds).map(|(l,h)| (l, l+h)).collect();

    type Maps = Vec<ReverseMap>;
    let s2s = Maps::new();
    let s2f = Maps::new();
    let f2w = Maps::new();
    let w2l = Maps::new();
    let l2t = Maps::new();
    let t2h = Maps::new();
    let h2l = Maps::new();

    let mut maps = [
        s2s, s2f, f2w, w2l, l2t, t2h, h2l
    ];

    let mut curr_map = 0;

    for line in data[2..].iter() {
        if line.contains("map") {
            continue;
        } else if line.is_empty() {
            curr_map += 1;
            continue;
        } 
        let map = &mut maps[curr_map];
        let dsr: Vec<u64> = line.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
        let dest = dsr[0];
        let src = dsr[1];
        let rng = dsr[2];

        map.push(ReverseMap::new(dest, src, rng));
    }
    let mut loc: u64 = 0;
    for i in 0.. {
        // if i % 100 == 0 {
        //     println!("{i}");
        // }
        let mut seed = i;

        for map in maps.iter().rev() {
            if let Some(s) = map.iter().fold(None, |maybe, curr_map| {
                if maybe.is_some() {
                    maybe
                } else {
                    curr_map.mapping(seed)
                }
            }) {
                seed = s;
            }
        }

        if seed_ranges.iter().any(|(low, high)| *low <= seed && seed <= *high) {
            loc = i;
            break;
        }
    }
    // For some reason there was a bug with the final output where it was one higher than the real answer
    println!("Part 2 = {}", loc);
}

#[derive(Debug)]
struct ReverseMap {
    dest_low: u64,
    dest_high: u64,
    src: u64
}

impl ReverseMap {
    fn new(from: u64, to: u64, rnge: u64) -> Self {
        ReverseMap {
            dest_low: from,
            dest_high: from + rnge,
            src: to
        }
    }

    fn mapping(&self, n: u64) -> Option<u64> {
        if self.dest_low <= n && n <= self.dest_high {
            Some(self.src + n - self.dest_low)
        } else {
            None
        }
    }
}