// --- Part Two ---

// Out of curiosity, the debugger would also like to know the size of the loop: starting from a state that has already been seen, how many block redistribution cycles must be performed before that same state is seen again?

// In the example above, 2 4 1 2 is seen again after four cycles, and so the answer in that example would be 4.

// How many cycles are in the infinite loop that arises from the configuration in your puzzle input?

// Input:
/*
4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3
*/

use std::io::{self, Read};
use std::cmp::{Ordering};
use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
struct Blocks {
    current: Vec<usize>,
    seen: HashMap<Vec<usize>, usize>
}

impl Blocks {
    fn new(first: Vec<usize>) -> Self {
        Self {
            current: first,
            seen: HashMap::new()
        }
    }
    fn redistribute(current: &Vec<usize>) -> Vec<usize> {
        let (drain_ind, &drain_amount) = current.iter().enumerate()
            .max_by(|&(i1, &x1), &(i2, &x2)| {
                match x1.cmp(&x2) {
                    Ordering::Equal => i2.cmp(&i1),
                    ordering => ordering
                }
            })
            .unwrap();

        let len = current.len();
        
        current.iter().enumerate().map(|(i, x)| {
            let inc = {
                let div = drain_amount / len;
                let rem = drain_amount % len;

                if (i + len - drain_ind - 1) % len < rem {
                    div + 1
                } else {
                    div
                }
            };

            if i == drain_ind {
                inc
            } else {
                x + inc
            }
        }).collect()
    }
}

impl Iterator for Blocks {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.seen.contains_key(&self.current) {
            None
        } else {
            let len = self.seen.len();
            self.seen.insert(self.current.clone(), len);
            let next = Blocks::redistribute(&mut self.current);
            let ret = mem::replace(&mut self.current, next);
            Some(ret)
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let arr: Vec<usize> = buffer.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut blocks = Blocks::new(arr);
    let count = blocks.by_ref().count();
    let repeated = blocks.current;
    let ans = count - blocks.seen[&repeated];

    println!("{:?}", ans);
}