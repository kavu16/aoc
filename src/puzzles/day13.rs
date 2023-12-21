#[test]

fn test() {
    let data = String::from(
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    );

    assert_eq!(solve1(&data), 405);

    assert_eq!(solve2(&data), 400);
}

#[derive(Debug, Clone)]
struct Mirror {
    horizontal: Vec<Vec<char>>,
    vertical: Vec<Vec<char>>,
}

impl Mirror {
    fn new(input: Vec<Vec<char>>) -> Self {
        let cols = input[0].len();

        let mut v = Vec::new();
        for c in 0..cols {
            v.push(
                input.iter()
                    .map(|row| row[c])
                    .collect::<Vec<char>>());
        }
        Self { horizontal: input, vertical: v }
    }

    fn find_reflection(&self) -> u64 {
         //Find Horizontal Reflection
        let h_size = self.horizontal.len();
        let mut h_match_grid = vec![vec![false; h_size]; h_size];
        for i in 0..h_size {
            for j in i..h_size {
                if self.horizontal[i] == self.horizontal[j] {
                    h_match_grid[i][j] = true;
                    h_match_grid[j][i] = true;
                }
            }
        }
        
        for i in 1..h_match_grid.len() {
            if h_match_grid[0][i] {
                let mut l = 0;
                let mut r = i;

                while l < r && h_match_grid[l][r] {
                    l += 1;
                    r -= 1;
                }
                if l > r {
                    return (100 * l) as u64;
                }
            }
        }

        for i in 1..h_match_grid.len() {
            if h_match_grid[i][h_match_grid.len()-1] {
                let mut l = i;
                let mut r = h_match_grid.len()-1;

                while l < r && h_match_grid[l][r] {
                    l += 1;
                    r -= 1;
                }
                if l > r {
                    return (100 * l) as u64;
                }
            }
        }

        //Find vertical reflection
        let v_size = self.vertical.len();
        let mut v_match_grid = vec![vec![false; v_size]; v_size];
        for i in 0..v_size {
            for j in i..v_size {
                if self.vertical[i] == self.vertical[j] {
                    v_match_grid[i][j] = true;
                    v_match_grid[j][i] = true;
                }
            }
        }

        for i in 1..v_match_grid.len() {
            if v_match_grid[0][i] {
                let mut l = 0;
                let mut r = i;

                while l < r && v_match_grid[l][r] {
                    l += 1;
                    r -= 1;
                }
                if l > r {
                    return l as u64;
                }
            }
        }

        for i in 1..v_match_grid.len() {
            if v_match_grid[i][v_match_grid.len()-1] {
                let mut l = i;
                let mut r = v_match_grid.len()-1;

                while l < r && v_match_grid[l][r] {
                    l += 1;
                    r -= 1;
                }
                if l > r {
                    return l as u64;
                }
            }
        }

        0
    }

    fn find_smudge(&self) -> u64 {
        //Find Horizontal smudge
        let h_size = self.horizontal.len();
        let mut h_diff_grid = vec![vec![2; h_size]; h_size];
        for i in 0..h_size {
            for j in (i+1)..h_size {
                h_diff_grid[i][j] = self.horizontal[i].iter().zip(self.horizontal[j].iter())
                                            .map(|(l, r)| usize::from(!(l == r)))
                                            .sum::<usize>();
                
            }
        }
        
        for i in 1..h_diff_grid.len() {
            let mut l = 0;
            let mut r = i;
            let mut diff_sum = 0;
            
            while l < r && diff_sum <= 1 {
                diff_sum += h_diff_grid[l][r];
                l += 1;
                r -= 1;
            }
            if l > r && diff_sum == 1 {
                return (100 * l) as u64;
            }
        }

        for i in 1..h_diff_grid.len() {
            let mut l = i;
            let mut r = h_diff_grid.len()-1;
            let mut diff_sum = 0;

            while l < r && diff_sum <= 1 {
                diff_sum += h_diff_grid[l][r];
                l += 1;
                r -= 1;
            }
            if l > r && diff_sum == 1 {
                return (100 * l) as u64;
            }
        }

        //Find vertical smudge
        let v_size = self.vertical.len();
        let mut v_diff_grid = vec![vec![2; v_size]; v_size];
        for i in 0..v_size {
            for j in (i+1)..v_size {
                v_diff_grid[i][j] = self.vertical[i].iter().zip(self.vertical[j].iter())
                                            .map(|(l, r)| usize::from(!(l == r)))
                                            .sum::<usize>();
            }
        }

        for i in 1..v_diff_grid.len() {
            let mut l = 0;
            let mut r = i;
            let mut diff_sum = 0;

            while l < r && diff_sum <= 1 {
                diff_sum += v_diff_grid[l][r];
                l += 1;
                r -= 1;
            }
            if l > r && diff_sum == 1 {
                return l as u64;
            }
        }

        for i in 1..v_diff_grid.len() {
            let mut l = i;
            let mut r = v_diff_grid.len()-1;
            let mut diff_sum = 0;

            while l < r && diff_sum <= 1 {
                diff_sum += v_diff_grid[l][r];
                l += 1;
                r -= 1;
            }
            if l > r && diff_sum == 1 {
                return l as u64;
            }
        }

        0
   }

}

pub fn solve1(data: &String) -> u64 {
    // let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    for problem in data.trim().split("\n\n") {
        let mut mirror: Vec<Vec<char>> = Vec::new();

        for line in problem.lines() {
            mirror.push(line.chars().collect::<Vec<char>>());
        }
        let m: Mirror = Mirror::new(mirror.clone());
        res += m.find_reflection();
    }

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> u64 {
    // let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    for problem in data.trim().split("\n\n") {
        let mut mirror: Vec<Vec<char>> = Vec::new();

        for line in problem.lines() {
            mirror.push(line.chars().collect::<Vec<char>>());
        }
        let m: Mirror = Mirror::new(mirror.clone());
        res += m.find_smudge();
    }

    println!("Part 2 = {}", res);
    res
}