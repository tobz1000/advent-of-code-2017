#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]
#![feature(range_contains)]

use std::env;
use std::io::Read;
use std::fs::File;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let problems = env::args().skip(1).flat_map(|a| {
        let vals: Vec<u8> = a.split('-').map(|n| n.parse().unwrap()).collect();
        
        match vals.len() {
            1 => vec![(vals[0], 1), (vals[0], 2)],
            2 => vec![(vals[0], vals[1])],
            _ => panic!()
        }.into_iter()
    });

    for (day, part) in problems {
        let solve = match (day, part) {
            (1, 1) => day1::part1,
            (1, 2) => day1::part2,
            (2, 1) => day2::part1,
            (2, 2) => day2::part2,
            (3, 1) => day3::part1,
            (3, 2) => day3::part2,
            (4, 1) => day4::part1,
            (4, 2) => day4::part2,
            (5, 1) => day5::part1,
            (5, 2) => day5::part2,
            (6, 1) => day6::part1,
            (6, 2) => day6::part2,
            // (7, 1) => day7::part1,
            (_, _) => panic!()
        };

        let mut input_file = File::open(format!("day{}-input.txt", day)).unwrap();
        let mut input_str = String::new();
        input_file.read_to_string(&mut input_str).unwrap();

        println!("{}", solve(input_str.as_str()));
    }
}