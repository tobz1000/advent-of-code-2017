// --- Day 12: Digital Plumber ---

// Walking along the memory banks of the stream, you find a small village that is experiencing a little confusion: some programs can't communicate with each other.

// Programs in this village communicate using a fixed system of pipes. Messages are passed between programs using these pipes, but most programs aren't connected to each other directly. Instead, programs pass messages between each other until the message reaches the intended recipient.

// For some reason, though, some of these messages aren't ever reaching their intended recipient, and the programs suspect that some pipes are missing. They would like you to investigate.

// You walk through the village and record the ID of each program and the IDs with which it can communicate directly (your puzzle input). Each program has one or more programs with which it can communicate, and these pipes are bidirectional; if 8 says it can communicate with 11, then 11 will say it can communicate with 8.

// You need to figure out how many programs are in the group that contains program ID 0.

// For example, suppose you go door-to-door like a travelling salesman and record the following list:

// 0 <-> 2
// 1 <-> 1
// 2 <-> 0, 3, 4
// 3 <-> 2, 4
// 4 <-> 2, 3, 6
// 5 <-> 6
// 6 <-> 4, 5

// In this example, the following programs are in the group that contains program ID 0:

//     Program 0 by definition.
//     Program 2, directly connected to program 0.
//     Program 3 via program 2.
//     Program 4 via program 2.
//     Program 5 via programs 6, then 4, then 2.
//     Program 6 via programs 4, then 2.

// Therefore, a total of 6 programs are in this group; all but program 1, which has a pipe that connects it to itself.

// How many programs are in the group that contains program ID 0?
extern crate regex;

use std::collections::HashSet;
use self::regex::Regex;

lazy_static! {
    static ref REG_PARSE_CONNECTIONS: Regex = Regex::new(
        r"^(?P<src>\d*) <-> (?P<dests>\d*(?:, \d*)*)$"
    ).unwrap();
}

struct Networks(Vec<HashSet<u32>>);

impl Networks {
    fn from_input(input: &str) -> Self {
        let mut networks = Networks(Vec::new());

        for line in input.split('\n') {
            let caps = REG_PARSE_CONNECTIONS.captures(line).unwrap();
            let src = caps["src"].parse().unwrap();
            let dests_str = caps["dests"].to_string();
            let dests = dests_str.split(", ").map(|s| s.parse().unwrap());

            let mut conns: HashSet<u32> = dests.collect();
            conns.insert(src);

            networks = networks.plus_conns(conns);
        }

        networks
    }

    fn plus_conns(self, mut conns: HashSet<u32>) -> Self {
        let mut new = Vec::new();

        for network in self.0.into_iter() {
            if network.intersection(&conns).next() != None {
                conns.extend(network.into_iter());
            } else {
                new.push(network);
            }
        }

        new.push(conns);

        Networks(new)
    }
}

pub fn part1(input: &str) -> String {
    let networks = Networks::from_input(input);

    let ans = networks.0.iter()
        .find(|&network| (*network).contains(&0))
        .unwrap()
        .len();

    ans.to_string()
}

// --- Part Two ---

// There are more programs than just the ones in the group containing program ID 0. The rest of them have no way of reaching that group, and still might have no way of reaching each other.

// A group is a collection of programs that can all communicate via pipes either directly or indirectly. The programs you identified just a moment ago are all part of the same group. Now, they would like you to determine the total number of groups.

// In the example above, there were 2 groups: one consisting of programs 0,2,3,4,5,6, and the other consisting solely of program 1.

// How many groups are there in total?
pub fn part2(input: &str) -> String {
    let networks = Networks::from_input(input);

    let ans = networks.0.len();

    ans.to_string()
}