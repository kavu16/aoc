#[test]

fn test() {
    let example1 = String::from(
"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
    );

    println!("Example 1:");
    assert_eq!(solve1::<7, 27>(&example1), 2);
    assert_eq!(solve2(&example1), 47);
}

pub fn solve1<const LOWER: i64, const UPPER: i64>(data: &String) -> usize {
    let mut hail = Vec::<(Vec<f64>, Vec<f64>)>::new();
    for line in data.lines() {
        let (pos, vel) = line.split_once('@').unwrap();
        let pos: Vec<f64> = pos.split(",").map(|n| n.trim().parse::<f64>().unwrap()).collect();
        let vel: Vec<f64> = vel.split(",").map(|n| n.trim().parse::<f64>().unwrap()).collect();

        hail.push((pos, vel));
    }

    // let lower = 200000000000000f64;
    // let upper = 400000000000000f64;
    let mut res = 0;
    for i in 0..hail.len() {
        for j in (i+1)..hail.len() {
            let (p0, v0) = &hail[i];
            let (p1, v1) = &hail[j];

            let m0 = v0[1] / v0[0];
            let m1 = v1[1] / v1[0];
            let x = (m0 * p0[0] - m1 * p1[0] + p1[1] - p0[1]) / (m0 - m1);
            let y = m0 * (x - p0[0]) + p0[1];

            // println!("Hail #{i} and Hail #{j} intersect at ({x}, {y})");
            if (x - p0[0]) / v0[0] < 0.0 || (x - p1[0]) / v1[0] < 0.0 {
                // println!("In the past");
                continue;
            }

            if LOWER as f64 <= x && x <= UPPER as f64 && LOWER as f64 <= y && y <= UPPER as f64 { res += 1; }
        }
    }

    println!("Part 1 = {}", res);
    res
}

use std::ops::{Add, Sub};
#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn dot(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn cross(&self, v: Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: -(self.x * v.z - self.z * v.x),
            z: self.x * v.y - self.y * v.x
        }
    }

    fn scalar(&self, f: f64) -> Vector {
        Vector {
            x: f * self.x,
            y: f * self.y,
            z: f * self.z
        }
    }

    fn taxi(&self) -> f64 {
        self.x + self.y + self.z
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add for &Vector {
    type Output = <Vector as Add>::Output;
    fn add(self, rhs: &Vector) -> Self::Output {
        Vector::add(*self, *rhs)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub for &Vector {
    type Output = <Vector as Add>::Output;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector::sub(*self, *rhs)
    }
}

impl FromIterator<f64> for Vector {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        let mut xyz = iter.into_iter().take(3);

        let x = xyz.next().unwrap();
        let y = xyz.next().unwrap();
        let z = xyz.next().unwrap();

        Vector { x, y, z }
    }
}

pub fn solve2(data: &String) -> usize {
    let mut hail = Vec::<(Vector, Vector)>::new();
    for line in data.lines(){
        let (p, v) = line.split_once('@').unwrap();
        let p = p.split(',').map(|n| n.trim().parse::<f64>().unwrap()).collect::<Vector>();
        let v = v.split(',').map(|n| n.trim().parse::<f64>().unwrap()).collect::<Vector>();
        hail.push((p, v));
    }

    let mut points = Vec::<Vector>::new();
    let mut velocities = Vec::<Vector>::new();
    for (p, v) in hail.iter().take(3) {
        points.push(*p);
        velocities.push(*v);
    }

    let (p0, p1, p2, v0, v1, v2) = (points[0], points[1], points[2], velocities[0], velocities[1], velocities[2]);
    let pp1 = p1 - p0;
    let pp2 = p2 - p0;
    let vv1 = v1 - v0;
    let vv2 = v2 - v0;

    let t1 = - ((pp1.cross(pp2)).dot(vv2))/((vv1.cross(pp2)).dot(vv2));
    let t2 = - ((pp1.cross(pp2)).dot(vv1))/((pp1.cross(vv2)).dot(vv1));

    let c1 = p1 + v1.scalar(t1);
    let c2 = p2 + v2.scalar(t2);
    let v = (c2 - c1).scalar(1.0/(t2 - t1));
    let p = c1 - v.scalar(t1);
    println!("Initial Rock = {:?}", p);

    let res = p.taxi() as usize;
    println!("Part 2 = {}", res);
    res
}