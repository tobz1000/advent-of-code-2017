// --- Day 6: Memory Reallocation ---

// A debugger program here is having an issue: it is trying to repair a memory reallocation routine, but it keeps getting stuck in an infinite loop.

// In this area, there are sixteen memory banks; each memory bank can hold any number of blocks. The goal of the reallocation routine is to balance the blocks between the memory banks.

// The reallocation routine operates in cycles. In each cycle, it finds the memory bank with the most blocks (ties won by the lowest-numbered memory bank) and redistributes those blocks among the banks. To do this, it removes all of the blocks from the selected bank, then moves to the next (by index) memory bank and inserts one of the blocks. It continues doing this until it runs out of blocks; if it reaches the last memory bank, it wraps around to the first one.

// The debugger would like to know how many redistributions can be done before a blocks-in-banks configuration is produced that has been seen before.

// For example, imagine a scenario with only four memory banks:

//     The banks start with 0, 2, 7, and 0 blocks. The third bank has the most blocks, so it is chosen for redistribution.
//     Starting with the next bank (the fourth bank) and then continuing to the first bank, the second bank, and so on, the 7 blocks are spread out over the memory banks. The fourth, first, and second banks get two blocks each, and the third bank gets one back. The final result looks like this: 2 4 1 2.
//     Next, the second bank is chosen because it contains the most blocks (four). Because there are four memory banks, each gets one block. The result is: 3 1 2 3.
//     Now, there is a tie between the first and fourth memory banks, both of which have three blocks. The first bank wins the tie, and its three blocks are distributed evenly over the other three banks, leaving it with none: 0 2 3 4.
//     The fourth bank is chosen, and its four blocks are distributed such that each of the four banks receives one: 1 3 4 1.
//     The third bank is chosen, and the same thing happens: 2 4 1 2.

// At this point, we've reached a state we've seen before: 2 4 1 2 was already seen. The infinite loop is detected after the fifth block redistribution cycle, and so the answer in this example is 5.

// Given the initial block counts in your puzzle input, how many redistribution cycles must be completed before a configuration is produced that has been seen before?

// Input:
/*
4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3
*/

use std::io::{self, Read};
use std::cmp::{Ordering};
use std::collections::HashSet;
use std::mem;

#[derive(Debug)]
struct Blocks {
    current: Vec<usize>,
    seen: HashSet<Vec<usize>>
}

impl Blocks {
    fn new(first: Vec<usize>) -> Self {
        Self {
            current: first,
            seen: HashSet::new()
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
        if self.seen.contains(&self.current) {
            None
        } else {
            self.seen.insert(self.current.clone());
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

    let ans = Blocks::new(arr).count();

    println!("{}", ans);
}