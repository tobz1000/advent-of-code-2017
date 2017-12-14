// --- Day 11: Hex Ed ---

// Crossing the bridge, you've barely reached the other side of the stream when a program comes up to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite grid!"

// Fortunately for her, you have plenty of experience with infinite grids.

// Unfortunately for you, it's a hex grid.

// The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the north, northeast, southeast, south, southwest, and northwest:

//   \ n  /
// nw +--+ ne
//   /    \
// -+      +-
//   \    /
// sw +--+ se
//   / s  \

// You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A "step" means to move from the hex you are in to any adjacent hex.)

// For example:

//     ne,ne,ne is 3 steps away.
//     ne,ne,sw,sw is 0 steps away (back where you started).
//     ne,ne,s,s is 2 steps away (se,se).
//     se,sw,se,sw,sw is 3 steps away (s,s,sw).
use std::str::FromStr;
use std::cmp::max;

use self::DirRelation::*;

enum DirRelation {
    Adv,
    Rev,
    LeftAdv,
    LeftRev,
    RightAdv,
    RightRev
}

#[derive(Debug)]
struct Direction(i32);

const NORTH: Direction = Direction(0);
const NORTHEAST: Direction = Direction(1);
const SOUTHEAST: Direction = Direction(2);
const SOUTH: Direction = Direction(3);
const SOUTHWEST: Direction = Direction(4);
const NORTHWEST: Direction = Direction(5);

impl Direction {
    fn relation(&self, other: &Direction) -> DirRelation {
        match (other.0 - self.0 + 6) % 6 {
            0 => Adv,
            1 => RightAdv,
            2 => RightRev,
            3 => Rev,
            4 => LeftRev,
            5 => LeftAdv,
            _ => panic!("{}")
        }
    }

    fn left(self) -> Self {
        Direction((self.0 - 1 + 6) % 6)
    }

    fn right(self) -> Self {
        Direction((self.0 + 1) % 6)
    }
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(NORTH),
            "ne" => Ok(NORTHEAST),
            "se" => Ok(SOUTHEAST),
            "s" => Ok(SOUTH),
            "sw" => Ok(SOUTHWEST),
            "nw" => Ok(NORTHWEST),
            s @ _ => Err(format!("Invalid direction {}", s))
        }
    }
}

#[derive(Debug)]
enum HexStep {
    Neutral,
    Offset {
        left_bearing: Direction,
        left: i32,
        right: i32
    }
}

impl HexStep {
    fn step(self, dir: Direction) -> Self {
        if let HexStep::Offset { mut left_bearing, mut left, mut right } = self {
            let (left_inc, right_inc) = match left_bearing.relation(&dir) {
                Adv => (1, 0),
                Rev => (-1, 0),
                LeftAdv => (1, -1),
                LeftRev => (0, -1),
                RightAdv => (0, 1),
                RightRev => (-1, 1)
            };

            left += left_inc;
            right += right_inc;

            if (left, right) == (0, 0) {
                return HexStep::Neutral;
            }

            let left_in_range = left > 0;
            let right_in_range = right > -1;

            // Pivot our bearing if either left or right component is now
            // out of range
            match (left_in_range, right_in_range) {
                (true, true) => (),
                (true, false) => {
                    left_bearing = left_bearing.left();
                    right = left - 1;
                    left = 1;
                },
                (false, true) => {
                    left_bearing = left_bearing.right();
                    left = right;
                    right = 0;
                },
                (false, false) => panic!()
            }

            HexStep::Offset { left_bearing, left, right }
        } else {
            HexStep::Offset { left_bearing: dir, left: 1, right: 0 }
        }
    }

    fn distance(&self) -> i32 {
            match *self {
            HexStep::Neutral => 0,
            HexStep::Offset { left, right, .. } => left + right
        }
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut position = HexStep::Neutral;
    let mut max_distance = position.distance();

    for step in input.split(',') {
        position = position.step(step.parse().unwrap());
        max_distance = max(max_distance, position.distance());
    }

    (position.distance(), max_distance)
}


pub fn part1(input: &str) -> String {
    let (final_dist, _max_dist) = solve(input);
    final_dist.to_string()
}

// --- Part Two ---

// How many steps away is the furthest he ever got from his starting position?
pub fn part2(input: &str) -> String {
    let (_final_dist, max_dist) = solve(input);
    max_dist.to_string()
}