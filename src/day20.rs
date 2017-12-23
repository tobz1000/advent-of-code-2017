// --- Day 20: Particle Swarm ---

// Suddenly, the GPU contacts you, asking for help. Someone has asked it to simulate too many particles, and it won't be able to finish them all in time to render the next frame at this rate.

// It transmits to you a buffer (your puzzle input) listing each particle in order (starting with particle 0, then particle 1, particle 2, and so on). For each particle, it provides the X, Y, and Z coordinates for the particle's position (p), velocity (v), and acceleration (a), each in the format <X,Y,Z>.

// Each tick, all particles are updated simultaneously. A particle's properties are updated in the following order:

//     Increase the X velocity by the X acceleration.
//     Increase the Y velocity by the Y acceleration.
//     Increase the Z velocity by the Z acceleration.
//     Increase the X position by the X velocity.
//     Increase the Y position by the Y velocity.
//     Increase the Z position by the Z velocity.

// Because of seemingly tenuous rationale involving z-buffering, the GPU would like to know which particle will stay closest to position <0,0,0> in the long term. Measure this using the Manhattan distance, which in this situation is simply the sum of the absolute values of a particle's X, Y, and Z position.

// For example, suppose you are only given two particles, both of which stay entirely on the X-axis (for simplicity). Drawing the current states of particles 0 and 1 (in that order) with an adjacent a number line and diagram of current X positions (marked in parenthesis), the following would take place:

// p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
// p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>                         (0)(1)

// p=< 4,0,0>, v=< 1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
// p=< 2,0,0>, v=<-2,0,0>, a=<-2,0,0>                      (1)   (0)

// p=< 4,0,0>, v=< 0,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
// p=<-2,0,0>, v=<-4,0,0>, a=<-2,0,0>          (1)               (0)

// p=< 3,0,0>, v=<-1,0,0>, a=<-1,0,0>    -4 -3 -2 -1  0  1  2  3  4
// p=<-8,0,0>, v=<-6,0,0>, a=<-2,0,0>                         (0)   

// At this point, particle 1 will never be closer to <0,0,0> than particle 0, and so, in the long run, particle 0 will stay closest.

// Which particle will stay closest to position <0,0,0> in the long term?
extern crate regex;

use std::collections::HashMap;
use std::ops::AddAssign;
use std::error::Error;
use std::str::FromStr;
use self::regex::Regex;

lazy_static! {
    static ref REG_PARSE_POINT: Regex = Regex::new(r"(?x)^
        p=<(?P<p0>-?\d*),(?P<p1>-?\d*),(?P<p2>-?\d*)>,
        \s
        v=<(?P<v0>-?\d*),(?P<v1>-?\d*),(?P<v2>-?\d*)>,
        \s
        a=<(?P<a0>-?\d*),(?P<a1>-?\d*),(?P<a2>-?\d*)>
    $").unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec3([i32; 3]);

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

#[derive(Debug)]
struct Point {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3
}

impl Point {
    fn diverging(&self) -> bool {
        (0..2).all(|dim| {
            let p_dir = self.pos.0[dim].signum();
            let v_dir = self.vel.0[dim].signum();
            let a_dir = self.acc.0[dim].signum();

            p_dir == v_dir && (v_dir == a_dir || a_dir == 0) 
        })
    }

    fn step(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }

    fn distance(&self) -> i32 {
        self.pos.0[0].abs() + self.pos.0[1].abs() + self.pos.0[2].abs()
    }
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = REG_PARSE_POINT.captures(s).ok_or("Regex capture failed")?;
        let pos = Vec3([
            captures["p0"].parse()?,
            captures["p1"].parse()?,
            captures["p2"].parse()?
        ]);
        let vel = Vec3([
            captures["v0"].parse()?,
            captures["v1"].parse()?,
            captures["v2"].parse()?
        ]);
        let acc = Vec3([
            captures["a0"].parse()?,
            captures["a1"].parse()?,
            captures["a2"].parse()?
        ]);

        Ok(Self { pos, vel, acc })
    }
}

pub fn part1(input: &str) -> String {
    let mut points: Vec<Point> = input.split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    while !points.iter().all(|point| point.diverging()) {
        for point in points.iter_mut() {
            point.step();
        }
    }

    let (closest_i, _closest_dist) = points.iter()
        .map(|point| point.distance())
        .enumerate()
        .min_by_key(|&(_i, d)| d)
        .unwrap();  
  
    closest_i.to_string()
}

// --- Part Two ---

// To simplify the problem further, the GPU would like to remove any particles that collide. Particles collide if their positions ever exactly match. Because particles are updated simultaneously, more than two particles can collide at the same time and place. Once particles collide, they are removed and cannot collide with anything else after that tick.

// For example:

// p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
// p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
// p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>    (0)   (1)   (2)            (3)
// p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>

// p=<-3,0,0>, v=< 3,0,0>, a=< 0,0,0>    
// p=<-2,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
// p=<-1,0,0>, v=< 1,0,0>, a=< 0,0,0>             (0)(1)(2)      (3)   
// p=< 2,0,0>, v=<-1,0,0>, a=< 0,0,0>

// p=< 0,0,0>, v=< 3,0,0>, a=< 0,0,0>    
// p=< 0,0,0>, v=< 2,0,0>, a=< 0,0,0>    -6 -5 -4 -3 -2 -1  0  1  2  3
// p=< 0,0,0>, v=< 1,0,0>, a=< 0,0,0>                       X (3)      
// p=< 1,0,0>, v=<-1,0,0>, a=< 0,0,0>

// ------destroyed by collision------    
// ------destroyed by collision------    -6 -5 -4 -3 -2 -1  0  1  2  3
// ------destroyed by collision------                      (3)         
// p=< 0,0,0>, v=<-1,0,0>, a=< 0,0,0>

// In this example, particles 0, 1, and 2 are simultaneously destroyed at the time and place marked X. On the next tick, particle 3 passes through unharmed.

// How many particles are left after all collisions are resolved?
fn remove_dup_pos(points: Vec<Point>) -> Vec<Point> {
    let mut lookup = HashMap::new();

    for point in points {
        lookup.entry(point.pos).or_insert(Vec::new()).push(point);
    }

    lookup.values_mut()
        .filter(|points| points.len() == 1)
        .map(|points| points.pop().unwrap())
        .collect()
}

pub fn part2(input: &str) -> String {
    let mut points = input.split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let ans = loop {
        points = remove_dup_pos(points);

        if points.iter().all(|point| point.diverging()) {
            break points.len();
        }

        for point in points.iter_mut() {
            point.step();
        }
    };

    ans.to_string()
}