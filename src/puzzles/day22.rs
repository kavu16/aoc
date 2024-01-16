use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
#[test]

fn test() {
    let example1 = String::from(
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
    );

    assert_eq!(solve1(&example1), 5);
    assert_eq!(solve2(&example1), 0);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Orientation {
    XAxis,
    YAxis,
    ZAxis,
}
use Orientation::*;

struct Brick {
    id: i32, 
    orientation: Orientation,
    begin: (i32, i32, i32),
    end: (i32, i32, i32),
    support: HashSet<i32>,
}

impl Brick {
    fn new(id: i32, begin: Vec<i32>, end: Vec<i32>) -> Self {
        Brick {
            id: id,
            orientation: {
                if begin[0] != end[0] { XAxis }
                else if begin[1] != end[1] { YAxis }
                else { ZAxis }
            },
            begin: (min(begin[0], end[0]), min(begin[1], end[1]), min(begin[2], end[2])), 
            end: (max(begin[0], end[0]), max(begin[1], end[1]), max(begin[2], end[2])),
            support: HashSet::<i32>::new(),
        }
    }

    fn fall(&mut self, grid: &mut Vec<i32>, xsize: i32, ysize: i32) {
        let x = self.begin.0;
        let y = self.begin.1;
        match self.orientation {
            ZAxis => {
                while self.begin.2 > 0 && grid[(x + xsize * y + xsize * ysize * (self.begin.2 - 1)) as usize] == 0 {
                    grid[(x + xsize * y + xsize * ysize * self.end.2) as usize] = 0;
                    grid[(x + xsize * y + xsize * ysize * (self.begin.2 - 1)) as usize] = self.id;
                    self.begin.2 -= 1;
                    self.end.2 -= 1;
                }
                for z in self.begin.2..(self.end.2 + 1) {
                    grid[(x + xsize * y + xsize * ysize * z) as usize] *= -1;
                }
                if self.begin.2 > 0 {
                    self.support.insert(grid[(x + xsize * y + xsize * ysize * (self.begin.2 - 1)) as usize].abs());
                }
            }
            YAxis => {
                let yend = self.end.1 + 1;
                'fall: while self.begin.2 > 0 {
                    for yy in y..yend {
                        if grid[(x + xsize * yy + xsize * ysize * (self.begin.2 -1)) as usize] != 0 {
                            break 'fall;
                        }
                    }
                    for yy in y..yend {
                        grid[(x + xsize * yy + xsize * ysize * self.begin.2) as usize] = 0;
                        grid[(x + xsize * yy + xsize * ysize * (self.begin.2 -1)) as usize] = self.id;
                    }
                    self.begin.2 -= 1;
                    self.end.2 -= 1;
                }
                for yy in y..yend {
                    grid[(x + xsize * yy + xsize * ysize * self.begin.2) as usize] *= -1;
                    if self.begin.2 > 0 && grid[(x + xsize * yy + xsize * ysize * (self.begin.2 - 1)) as usize] != 0 {
                        self.support.insert(grid[(x + xsize * yy + xsize * ysize * (self.begin.2 - 1)) as usize].abs());
                    }
                }
            }
            XAxis => {
                let xend = self.end.0 + 1;
                'fall: while self.begin.2 > 0 {
                    for xx in x..xend {
                        if grid[(xx + xsize * y + xsize * ysize * (self.begin.2 -1)) as usize] != 0 {
                            break 'fall;
                        }
                    }
                    for xx in x..xend {
                        grid[(xx + xsize * y + xsize * ysize * self.begin.2) as usize] = 0;
                        grid[(xx + xsize * y + xsize * ysize * (self.begin.2 -1)) as usize] = self.id;
                    }
                    self.begin.2 -= 1;
                    self.end.2 -= 1;
                }
                for xx in x..xend {
                    grid[(xx + xsize * y + xsize * ysize * self.begin.2) as usize] *= -1;
                    if self.begin.2 > 0 && grid[(xx + xsize * y + xsize * ysize * (self.begin.2 - 1)) as usize] != 0 {
                        self.support.insert(grid[(xx + xsize * y + xsize * ysize * (self.begin.2 - 1)) as usize].abs());
                    }
                }
            }
        }
    }

    fn unstable(&self) -> Option<&i32> {
        if self.support.len() == 1 {
            return self.support.iter().next();
        }
        None
    }
}

pub fn solve1(data: &String) -> usize {
    let mut bricks = HashMap::<i32, Brick>::new();
    let mut id_counter = 0;
    for line in data.lines() {
        id_counter += 1;
        let (b, e) = line.split_once("~").unwrap();
        let begin: Vec<i32> = b.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let end: Vec<i32> = e.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        bricks.insert(id_counter, Brick::new(id_counter, begin, end));
    }
    
    let maxx = bricks.iter().map(|(_id, brick)| max(brick.begin.0, brick.end.0)).max().unwrap();
    let maxy = bricks.iter().map(|(_id, brick)| max(brick.begin.1, brick.end.1)).max().unwrap();
    let maxz = bricks.iter().map(|(_id, brick)| max(brick.begin.2, brick.end.2)).max().unwrap();

    let xsize = maxx + 1;
    let ysize = maxy + 1;
    let zsize = maxz + 1;

    let mut grid = Vec::<i32>::new();
    grid.resize((xsize * ysize * zsize) as usize, 0);

    for (_id, Brick { 
                        id, 
                        orientation, 
                        begin: (x1, y1, z1), 
                        end: (x2, y2, z2),
                        support: _ }) in bricks.iter() {
        match orientation {
            XAxis => {
                for x in *x1..*x2+1 {
                    assert_eq!(grid[(x + xsize * y1 + xsize * ysize * z1) as usize], 0);
                    grid[(x + xsize * y1 + xsize * ysize * z1) as usize] = *id;
                }
            }
            YAxis => {
                for y in *y1..*y2+1 {
                    assert_eq!(grid[(x1 + xsize * y + xsize * ysize * z1) as usize], 0);
                    grid[(x1 + xsize * y + xsize * ysize * z1) as usize] = *id;
                }
            }
            ZAxis => {
                for z in *z1..*z2+1 {
                    assert_eq!(grid[(x1 + xsize * y1 + xsize * ysize * z) as usize], 0);
                    grid[(x1 + xsize * y1 + xsize * ysize * z) as usize] = *id;
                }
            }
        }
    }

    for i in 0..(xsize * ysize) {
        assert_eq!(grid[i as usize], 0);
    }

    for z in 0..zsize {
        for x in 0..xsize {
            for y in 0..ysize {
                let coord = grid[(x + xsize * y + xsize * ysize * z) as usize];
                if coord <= 0 {
                    continue;
                }
                bricks.get_mut(&coord).unwrap().fall(&mut grid, xsize, ysize);
            }
        }
    }

    let mut support_bricks = HashSet::<i32>::new();

    for z in 0..zsize {
        for x in 0..xsize {
            for y in 0..ysize {
                let mut id = grid[(x + xsize * y + xsize * ysize * z) as usize];
                if id >= 0 {
                    continue;
                }
                id = id.abs();
                if let Some(support) = bricks.get(&id).unwrap().unstable() {
                    support_bricks.insert(*support);
                }
            }
        }
    }

    let res = bricks.len() - support_bricks.len();
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(_data: &String) -> usize {
    let res = 0;
    println!("Part 2 = {}", res);
    res
}