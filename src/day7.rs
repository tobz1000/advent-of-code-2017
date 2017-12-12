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
extern crate itertools;

use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use self::regex::Regex;
use self::itertools::Itertools;

struct ProgramSingle {
    name: String,
    weight: usize,
    child_names: Vec<String>
}

lazy_static! {
    static ref REG_PARSE_PROGRAM: Regex = Regex::new(r"(?x)
        ^(?P<name>[a-z]*)
        \s
        \((?P<weight>\d*)\)
        (?:\s->\s(?P<children>(?:
            (?:[a-z]*)
            ,\s(?:[a-z]*)
        )*))?
    ").unwrap();
}

impl FromStr for ProgramSingle {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = REG_PARSE_PROGRAM.captures(s).unwrap();
        let name = captures["name"].to_string();
        let weight = captures["weight"].parse().unwrap();
        let child_names = match captures.name("children") {
            Some(m) => m.as_str().split(", ").map(|s| s.to_string()).collect(),
            None => vec![]
        };

        Ok(Self { name, weight, child_names })
    }
}

pub fn part1(input: &str) -> String {
    let mut maybe_bottom: HashSet<String> = HashSet::new();
    let mut not_bottom: HashSet<String> = HashSet::new();
    let progs = input.split('\n').map(|s| s.parse().unwrap());

    for ProgramSingle { name, child_names, .. } in progs {
        maybe_bottom.insert(name);
        not_bottom.extend(child_names.into_iter());
    }

    let bottom = &maybe_bottom - &not_bottom;

    assert!(bottom.len() == 1);

    let ans = bottom.into_iter().next().unwrap();

    ans.to_string()
}

// --- Part Two ---

// The programs explain the situation: they can't get down. Rather, they could get down, if they weren't expending all of their energy trying to keep the tower balanced. Apparently, one program has the wrong weight, and until it's fixed, they're stuck here.

// For any program holding a disc, each program standing on that disc forms a sub-tower. Each of those sub-towers are supposed to be the same weight, or the disc itself isn't balanced. The weight of a tower is the sum of the weights of the programs in that tower.

// In the example above, this means that for ugml's disc to be balanced, gyxo, ebii, and jptl must all have the same weight, and they do: 61.

// However, for tknk to be balanced, each of the programs standing on its disc and all programs above it must each match. This means that the following sums must all be the same:

//     ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
//     padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
//     fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243

// As you can see, tknk's disc is unbalanced: ugml's stack is heavier than the other two. Even though the nodes above ugml are balanced, ugml itself is too heavy: it needs to be 8 units lighter for its stack to weigh 243 and keep the towers balanced. If this change were made, its weight would be 60.

// Given that exactly one program is the wrong weight, what would its weight need to be to balance the entire tower?
#[derive(Debug)]
struct ProgramTree {
    name: String,
    weight: usize,
    total_weight: usize,
    balanced: bool,
    children: Vec<ProgramTree>,
}

impl ProgramTree {
    fn new(
        ProgramSingle { name, weight, child_names }: ProgramSingle,
        progs: &mut HashMap<String, ProgramSingle>
    ) -> Self {
        let children: Vec<ProgramTree> = child_names.iter().map(|name| {
            let child = progs.remove(name.as_str()).unwrap();
            ProgramTree::new(child, progs)
        }).collect();

        let total_weight = children.iter()
            .fold(weight, |acc, child| acc + child.total_weight);
        
        let balanced = children.iter()
            .map(|child| child.total_weight)
            .all_equal();

        ProgramTree { name, weight, balanced, total_weight, children }
    }
}

impl FromStr for ProgramTree {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut progs: HashMap<String, ProgramSingle> = HashMap::new();
        let mut maybe_bottom: HashSet<String> = HashSet::new();
        let mut not_bottom: HashSet<String> = HashSet::new();

        for line in s.split('\n') {
            let prog: ProgramSingle = line.parse()?;
            maybe_bottom.insert(prog.name.clone());
            not_bottom.extend(prog.child_names.clone());
            progs.insert(prog.name.clone(), prog);
        }

        let bottom_name = {
            let bottom_names = &maybe_bottom - &not_bottom;
            assert!(bottom_names.len() == 1);
            bottom_names.into_iter().next().unwrap()
        };

        let bottom_prog = progs.remove(&bottom_name).unwrap();

        Ok(ProgramTree::new(bottom_prog, &mut progs))
    }
}

pub fn part2(input: &str) -> String {
    let program_tree: ProgramTree = input.parse().unwrap();
    let mut last_unbalanced = &program_tree;

    loop {
        match last_unbalanced.children.iter().find(|p| !p.balanced) {
            Some(unbalanced) => { last_unbalanced = unbalanced; }
            None => { break; }
        }
    }

    let mut weights_lookup: HashMap<usize, Vec<usize>> = HashMap::new();

    for prog in last_unbalanced.children.iter() {
        let list = weights_lookup.entry(prog.total_weight)
            .or_insert(Vec::new());
        (*list).push(prog.weight);
    }

    assert_eq!(weights_lookup.len(), 2);

    let (unbalanced, balanced): (Vec<_>, Vec<_>) = weights_lookup.iter()
        .partition(|&(_total_weight, weights)| weights.len() == 1);

    let (&unbalanced_total_weight, unbalanced_weight) = unbalanced[0];
    let (&balanced_total_weight, _) = balanced[0];

    let ans = (unbalanced_weight[0] as isize)
        + (balanced_total_weight as isize)
        - (unbalanced_total_weight as isize);

    ans.to_string()
}