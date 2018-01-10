// --- Day 24: Electromagnetic Moat ---

// The CPU itself is a large, black building surrounded by a bottomless pit. Enormous metal tubes extend outward from the side of the building at regular intervals and descend down into the void. There's no way to cross, but you need to get inside.

// No way, of course, other than building a bridge out of the magnetic components strewn about nearby.

// Each component has two ports, one on each end. The ports come in all different types, and only matching types can be connected. You take an inventory of the components by their port types (your puzzle input). Each port is identified by the number of pins it uses; more pins mean a stronger connection for your bridge. A 3/7 component, for example, has a type-3 port on one side, and a type-7 port on the other.

// Your side of the pit is metallic; a perfect surface to connect a magnetic, zero-pin port. Because of this, the first port you use must be of type 0. It doesn't matter what type of port you end with; your goal is just to make the bridge as strong as possible.

// The strength of a bridge is the sum of the port types in each component. For example, if your bridge is made of components 0/3, 3/7, and 7/4, your bridge has a strength of 0+3 + 3+7 + 7+4 = 24.

// For example, suppose you had the following components:

// 0/2
// 2/2
// 2/3
// 3/4
// 3/5
// 0/1
// 10/1
// 9/10

// With them, you could make the following valid bridges:

//     0/1
//     0/1--10/1
//     0/1--10/1--9/10
//     0/2
//     0/2--2/3
//     0/2--2/3--3/4
//     0/2--2/3--3/5
//     0/2--2/2
//     0/2--2/2--2/3
//     0/2--2/2--2/3--3/4
//     0/2--2/2--2/3--3/5

// (Note how, as shown by 10/1, order of ports within a component doesn't matter. However, you may only use each port on a component once.)

// Of these bridges, the strongest one is 0/1--10/1--9/10; it has a strength of 0+1 + 1+10 + 10+9 = 31.

// What is the strength of the strongest bridge you can make with the components you have available?

// --- Part Two ---

// The bridge you've built isn't long enough; you can't jump the rest of the way.

// In the example above, there are two longest bridges:

//     0/2--2/2--2/3--3/4
//     0/2--2/2--2/3--3/5

// Of them, the one which uses the 3/5 component is stronger; its strength is 0+2 + 2+2 + 2+3 + 3+5 = 19.

// What is the strength of the longest bridge you can make? If you can make multiple bridges of the longest length, pick the strongest one.
use std::ops::Add;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct Subchain {
    strength: u32,
    length: u32,
}

impl Add for Subchain {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Subchain {
            strength: self.strength + rhs.strength,
            length: self.length + rhs.length
        }
    }
}

fn walk(
    port: u32,
    comps: &[(u32, u32)],
    cmp: fn(Subchain, Subchain) -> Ordering
) -> Subchain {
    let mut max_subchain = Subchain { strength: 0, length: 0 };

    for i in 0..comps.len() {
        let (a, b) = comps[i];

        let next_port = if port == a { 
            b
        } else if port == b {
            a
        } else {
            continue;
        };

        let mut next_comps = comps.to_vec();
        next_comps.swap_remove(i);

        let mut next_subchain = walk(next_port, &next_comps, cmp);
        next_subchain.strength += port;

        max_subchain = match cmp(max_subchain, next_subchain) {
            Ordering::Less => next_subchain,
            _ => max_subchain
        };
    }

    max_subchain + Subchain { strength: port, length: 1 }
}

fn solve(input: &str, cmp: fn(Subchain, Subchain) -> Ordering) -> Subchain {
    let comps: Vec<(u32, u32)> = input.split('\n').map(|line| {
        let mut ports = line.split('/');
        let port1 = ports.next().unwrap().parse().unwrap();
        let port2 = ports.next().unwrap().parse().unwrap();
        (port1, port2)
    }).collect();

    walk(0, &comps, cmp)
}

pub fn part1(input: &str) -> String {
    let strongest = solve(input, |a, b| a.strength.cmp(&b.strength));
    let ans = strongest.strength;
    ans.to_string()
}

pub fn part2(input: &str) -> String {
    let longest = solve(input, |a, b| {
        match a.length.cmp(&b.length) {
            Ordering::Equal => a.strength.cmp(&b.strength),
            c @ _ => c
        }
    });
    let ans = longest.strength;
    ans.to_string()
}