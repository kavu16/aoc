use std::collections::{HashMap, HashSet, BTreeSet};
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
    assert_eq!(solve2(&example1), 7);

    let example2 = String::from(
"0,1,1~2,1,1
0,1,2~0,1,3
2,1,2~2,1,3
0,1,4~2,1,4
1,0,5~1,2,5"
    );
    println!("Example 2:");
    assert_eq!(solve2(&example2), 5);

    let example3 = String::from(
"0,0,1~0,2,1
0,0,2~0,2,2"
    );
    println!("Example 3:");
    assert_eq!(solve2(&example3), 1);

    let example4 = String::from(
"1,1,1~1,1,2
1,0,3~1,2,3
0,0,4~2,0,4
0,2,4~2,2,4
1,0,5~1,2,5"
    );
    println!("Example 4:");
    assert_eq!(solve2(&example4), 7);

    let example5 = String::from(
"0,0,1~0,0,2
0,0,3~0,1,3
0,1,4~0,3,4
0,2,1~0,2,3
0,3,1~0,3,3
0,0,4~0,0,4
0,0,5~0,1,5
"
    );
    println!("Example 5:");
    assert_eq!(solve2(&example5), 3);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Orientation {
    XAxis,
    YAxis,
    ZAxis,
}
use Orientation::*;

#[derive(Debug, Clone)]
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

    fn unstable(&self, grid: &mut Vec<i32>, xsize: i32, ysize: i32) -> Option<&i32> {
        let x = self.begin.0;
        let y = self.begin.1;
        let z = self.begin.2;
        match self.orientation {
            ZAxis => {
                let zend = self.end.2 + 1;
                for zz in z..zend {
                    grid[(x + xsize * y + xsize * ysize * zz) as usize] *= -1;
                }
            }
            YAxis => {
                let yend = self.end.1 + 1;
                for yy in y..yend {
                    grid[(x + xsize * yy + xsize * ysize * z) as usize] *= -1;
                }
            }
            XAxis => {
                let xend = self.end.0 + 1;
                for xx in x..xend {
                    grid[(xx + xsize * y + xsize * ysize * z) as usize] *= -1;
                }
            }
        }
        if self.support.len() == 1 {
            return self.support.iter().next();
        }
        None
    }

    fn above(&self, grid: &Vec<i32>, xsize: i32, ysize: i32) -> HashSet<i32> {
        let mut above = HashSet::<i32>::new();
        let x = self.begin.0;
        let y = self.begin.1;
        let z = self.begin.2;
        match self.orientation {
            ZAxis => {
                let id = grid[(x + xsize * y + xsize * ysize * (self.end.2 + 1)) as usize];
                if id != 0 {
                    above.insert(id.abs());
                }
            }
            YAxis => {
                let yend = self.end.1 + 1;
                for yy in y..yend {
                    let id = grid[(x + xsize * yy + xsize * ysize * (z + 1)) as usize];
                    if id != 0 {
                        above.insert(id.abs());
                    }
                }
            }
            XAxis => {
                let xend = self.end.0 + 1;
                for xx in x..xend {
                    let id = grid[(xx + xsize * y + xsize * ysize * (z + 1)) as usize];
                    if id != 0 {
                        above.insert(id.abs());
                    }
                }
            }
        }
        above
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
    
    let xsize = bricks.iter().map(|(_id, brick)| max(brick.begin.0, brick.end.0)).max().unwrap() + 1;
    let ysize = bricks.iter().map(|(_id, brick)| max(brick.begin.1, brick.end.1)).max().unwrap() + 1;
    let zsize = bricks.iter().map(|(_id, brick)| max(brick.begin.2, brick.end.2)).max().unwrap() + 1;

    let mut grid = Vec::<i32>::new();
    grid.resize((xsize * ysize * zsize) as usize, 0);

    for (_id, Brick { 
                        id, 
                        orientation, 
                        begin: (x1, y1, z1), 
                        end: (x2, y2, z2),
                        support: _ 
                    }
                ) in bricks.iter() {
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
                if let Some(support) = bricks.get(&id).unwrap().unstable(&mut grid, xsize, ysize) {
                    support_bricks.insert(*support);
                }
            }
        }
    }
    
    let res = bricks.len() - support_bricks.len();
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let mut bricks = HashMap::<i32, Brick>::new();
    let mut id_counter = 0;
    for line in data.lines() {
        id_counter += 1;
        let (b, e) = line.split_once("~").unwrap();
        let begin: Vec<i32> = b.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let end: Vec<i32> = e.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        bricks.insert(id_counter, Brick::new(id_counter, begin, end));
    }
    
    let xsize = bricks.iter().map(|(_id, brick)| max(brick.begin.0, brick.end.0)).max().unwrap() + 1;
    let ysize = bricks.iter().map(|(_id, brick)| max(brick.begin.1, brick.end.1)).max().unwrap() + 1;
    let zsize = bricks.iter().map(|(_id, brick)| max(brick.begin.2, brick.end.2)).max().unwrap() + 1;

    let mut grid = Vec::<i32>::new();
    grid.resize((xsize * ysize * zsize) as usize, 0);

    for (_id, Brick { 
                        id, 
                        orientation, 
                        begin: (x1, y1, z1), 
                        end: (x2, y2, z2),
                        support: _ 
                    }
                ) in bricks.iter() {
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

    let mut supports = HashSet::<i32>::new();

    for z in 0..zsize {
        for x in 0..xsize {
            for y in 0..ysize {
                let id = grid[(x + xsize * y + xsize * ysize * z) as usize];
                if id >= 0 {
                    continue;
                }
                let id = id.abs();
                if let Some(support) = bricks.get(&id).unwrap().unstable(&mut grid, xsize, ysize) {
                    supports.insert(*support);
                }
            }
        }
    }

    let mut res = 0;

    for id in supports {
        let mut queue = BTreeSet::<i32>::new();
        let mut affected = HashSet::<i32>::new();
        queue.insert(id);
        affected.insert(id);
        while let Some(curr_id) = queue.pop_first() {
            for above_id in bricks.get(&curr_id).unwrap().above(&grid, xsize, ysize) {
                if bricks.get(&above_id).unwrap().support.iter().all(|s_id| affected.contains(s_id)) {
                    affected.insert(above_id);
                    queue.insert(above_id);
                }
            }
        }
        res += affected.len() - 1;
    }
    
    println!("Part 2 = {}", res);
    res
}