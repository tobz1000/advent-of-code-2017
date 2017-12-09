// --- Day 7: Recursive Circus ---

// Wandering further through the circuits of the computer, you come upon a tower of programs that have gotten themselves into a bit of trouble. A recursive algorithm has gotten out of hand, and now they're balanced precariously in a large tower.

// One program at the bottom supports the entire tower. It's holding a large disc, and on the disc are balanced several more sub-towers. At the bottom of these sub-towers, standing on the bottom disc, are other programs, each holding their own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many programs stand simply keeping the disc below them balanced but with no disc of their own.

// You offer to help, but first you need to understand the structure of these towers. You ask each program to yell out their name, their weight, and (if they're holding a disc) the names of the programs immediately above them balancing on that disc. You write this information down (your puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the time you're done, you're not sure which program gave which information.

// For example, if your list is the following:

// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)

// ...then you would be able to recreate the structure of the towers that looks like this:

//                 gyxo
//               /     
//          ugml - ebii
//        /      \     
//       |         jptl
//       |        
//       |         pbga
//      /        /
// tknk --- padx - havc
//      \        \
//       |         qoyq
//       |             
//       |         ktlj
//        \      /     
//          fwft - cntj
//               \     
//                 xhth

// In this example, tknk is at the bottom of the tower (the bottom program), and is holding up ugml, padx, and fwft. Those programs are, in turn, holding up other programs; in this example, none of those programs are holding up any other programs, and are all the tops of their own towers. (The actual tower balancing in front of you is much larger.)

// Before you're ready to help them, you need to make sure your information is correct. What is the name of the bottom program?
extern crate regex;

use self::regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

struct Program {
    name: String,
    weight: usize,
    children: Vec<String>,
}

impl FromStr for Program {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?x)
            ^(?P<name>[a-z]*)
            \s
            \((?P<weight>\d*)\)
            (?:\s->\s(?P<children>(?:
                (?:[a-z]*)
                ,\s(?:[a-z]*)
            )*))?
        ").unwrap();
        let captures = re.captures(s).unwrap();
        let name = captures["name"].to_string();
        let weight = captures["weight"].parse().unwrap();
        let children = match captures.name("children") {
            Some(m) => m.as_str().split(", ").map(|s| s.to_string()).collect(),
            None => vec![]
        };

        Ok(Program { name, weight, children })
    }
}

pub fn part1(input: &str) -> String {
    let mut maybe_bottom: HashSet<String> = HashSet::new();
    let mut not_bottom: HashSet<String> = HashSet::new();
    let progs = input.split('\n').map(|s| s.parse().unwrap());

    for Program { name, children, .. } in progs {
        maybe_bottom.insert(name);
        not_bottom.extend(children.into_iter());
    }

    let bottom = &maybe_bottom - &not_bottom;

    assert!(bottom.len() == 1);

    let ans = bottom.into_iter().next().unwrap();

    ans.to_string()
}