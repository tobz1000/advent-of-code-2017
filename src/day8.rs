// --- Day 8: I Heard You Like Registers ---

// You receive a signal directly from the CPU. Because of your recent assistance with jump instructions, it would like you to compute the result of a series of unusual register instructions.

// Each instruction consists of several parts: the register to modify, whether to increase or decrease that register's value, the amount by which to increase or decrease it, and a condition. If the condition fails, skip the instruction without modifying the register. The registers all start at 0. The instructions look like this:

// b inc 5 if a > 1
// a inc 1 if b < 5
// c dec -10 if a >= 1
// c inc -20 if c == 10

// These instructions would be processed as follows:

//     Because a starts at 0, it is not greater than 1, and so b is not modified.
//     a is increased by 1 (to 1) because b is less than 5 (it is 0).
//     c is decreased by -10 (to 10) because a is now greater than or equal to 1 (it is 1).
//     c is increased by -20 (to -10) because c is equal to 10.

// After this process, the largest value in any register is 1.

// You might also encounter <= (less than or equal to) or != (not equal to). However, the CPU doesn't have the bandwidth to tell you what all the registers are named, and leaves that to you to determine.

// What is the largest value in any register after completing the instructions in your puzzle input?
extern crate regex;

use std::collections::HashMap;
use self::regex::Regex;
use std::cmp::max;

struct Counter {
    hash_map: HashMap<String, i32>
}

impl Counter {
    fn get(&mut self, key: String) -> &mut i32 {
        self.hash_map.entry(key).or_insert(0)
    }
}

lazy_static! {
    static ref REG_PARSE_INSTR: Regex = Regex::new(
        r"^(?P<reg>[a-z]*) (?P<dir>inc|dec) (?P<cnt>-?\d*) if (?P<cmpreg>[a-z]*) (?P<cmp>[!=><]=|<|>) (?P<cmpcnt>-?\d*)$"
    ).unwrap();
}

fn solve(input: &str) -> (i32, i32) {
    let mut counter = Counter { hash_map: HashMap::new() };
    let mut max_val = None;

    for instr in input.split('\n') {
        let caps = REG_PARSE_INSTR.captures(instr).unwrap();

        let &mut cmp_reg = counter.get(caps["cmpreg"].to_string());
        let cmp = &caps["cmp"];
        let cmp_count = caps["cmpcnt"].parse().unwrap();

        let valid = match cmp {
            "!=" => cmp_reg != cmp_count,
            "==" => cmp_reg == cmp_count,
            ">=" => cmp_reg >= cmp_count,
            "<=" => cmp_reg <= cmp_count,
            ">" => cmp_reg > cmp_count,
            "<" => cmp_reg < cmp_count,
            _ => panic!()
        };

        if valid {
            let reg = counter.get(caps["reg"].to_string());
            let count: i32 = caps["cnt"].parse().unwrap();
            let dir = &caps["dir"];

            match dir {
                "inc" => *reg += count,
                "dec" => *reg -= count,
                _ => panic!()
            }

            max_val = match max_val {
                None => Some(*reg),
                Some(max_val) => Some(max(max_val, *reg))
            };
        }
    }

    let &final_max = counter.hash_map.values().max().unwrap();

    (final_max, max_val.unwrap())
}

pub fn part1(input: &str) -> String {
    let (final_max, _max_val) = solve(input);
    let ans = final_max;
    ans.to_string()
}

// --- Part Two ---

// To be safe, the CPU also needs to know the highest value held in any register during this process so that it can decide how much memory to allocate to these operations. For example, in the above instructions, the highest value ever held was 10 (in register c after the third instruction was evaluated).
pub fn part2(input: &str) -> String {
    let (_final_max, max_val) = solve(input);
    let ans = max_val;
    ans.to_string()
}