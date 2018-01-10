// --- Day 22: Sporifica Virus ---

// Diagnostics indicate that the local grid computing cluster has been contaminated with the Sporifica Virus. The grid computing cluster is a seemingly-infinite two-dimensional grid of compute nodes. Each node is either clean or infected by the virus.

// To prevent overloading the nodes (which would render them useless to the virus) or detection by system administrators, exactly one virus carrier moves through the network, infecting or cleaning nodes as it moves. The virus carrier is always located on a single node in the network (the current node) and keeps track of the direction it is facing.

// To avoid detection, the virus carrier works in bursts; in each burst, it wakes up, does some work, and goes back to sleep. The following steps are all executed in order one time each burst:

//     If the current node is infected, it turns to its right. Otherwise, it turns to its left. (Turning is done in-place; the current node does not change.)
//     If the current node is clean, it becomes infected. Otherwise, it becomes cleaned. (This is done after the node is considered for the purposes of changing direction.)
//     The virus carrier moves forward one node in the direction it is facing.

// Diagnostics have also provided a map of the node infection status (your puzzle input). Clean nodes are shown as .; infected nodes are shown as #. This map only shows the center of the grid; there are many more nodes beyond those shown, but none of them are currently infected.

// The virus carrier begins in the middle of the map facing up.

// For example, suppose you are given a map like this:

// ..#
// #..
// ...

// Then, the middle of the infinite grid looks like this, with the virus carrier's position marked with [ ]:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . # . . .
// . . . #[.]. . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// The virus carrier is on a clean node, so it turns left, infects the node, and moves left:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . # . . .
// . . .[#]# . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// The virus carrier is on an infected node, so it turns right, cleans the node, and moves up:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . .[.]. # . . .
// . . . . # . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// Four times in a row, the virus carrier finds a clean, infects it, turns left, and moves forward, ending in the same place and still facing up:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . #[#]. # . . .
// . . # # # . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// Now on the same node as before, it sees an infection, which causes it to turn right, clean the node, and move forward:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . # .[.]# . . .
// . . # # # . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// After the above actions, a total of 7 bursts of activity had taken place. Of them, 5 bursts of activity caused an infection.

// After a total of 70, the grid looks like this, with the virus carrier facing up:

// . . . . . # # . .
// . . . . # . . # .
// . . . # . . . . #
// . . # . #[.]. . #
// . . # . # . . # .
// . . . . . # # . .
// . . . . . . . . .
// . . . . . . . . .

// By this time, 41 bursts of activity caused an infection (though most of those nodes have since been cleaned).

// After a total of 10000 bursts of activity, 5587 bursts will have caused an infection.

// Given your actual map, after 10000 bursts of activity, how many bursts cause a node to become infected? (Do not count nodes that begin infected.)

// --- Part Two ---

// As you go to remove the virus from the infected nodes, it evolves to resist your attempt.

// Now, before it infects a clean node, it will weaken it to disable your defenses. If it encounters an infected node, it will instead flag the node to be cleaned in the future. So:

//     Clean nodes become weakened.
//     Weakened nodes become infected.
//     Infected nodes become flagged.
//     Flagged nodes become clean.

// Every node is always in exactly one of the above states.

// The virus carrier still functions in a similar way, but now uses the following logic during its bursts of action:

//     Decide which way to turn based on the current node:
//         If it is clean, it turns left.
//         If it is weakened, it does not turn, and will continue moving in the same direction.
//         If it is infected, it turns right.
//         If it is flagged, it reverses direction, and will go back the way it came.
//     Modify the state of the current node, as described above.
//     The virus carrier moves forward one node in the direction it is facing.

// Start with the same map (still using . for clean and # for infected) and still with the virus carrier starting in the middle and facing up.

// Using the same initial state as the previous example, and drawing weakened as W and flagged as F, the middle of the infinite grid looks like this, with the virus carrier's position again marked with [ ]:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . # . . .
// . . . #[.]. . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// This is the same as before, since no initial nodes are weakened or flagged. The virus carrier is on a clean node, so it still turns left, instead weakens the node, and moves left:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . # . . .
// . . .[#]W . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// The virus carrier is on an infected node, so it still turns right, instead flags the node, and moves up:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . .[.]. # . . .
// . . . F W . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// This process repeats three more times, ending on the previously-flagged node and facing right:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . W W . # . . .
// . . W[F]W . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// Finding a flagged node, it reverses direction and cleans the node:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . W W . # . . .
// . .[W]. W . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// The weakened node becomes infected, and it continues in the same direction:

// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . W W . # . . .
// .[.]# . W . . . .
// . . . . . . . . .
// . . . . . . . . .
// . . . . . . . . .

// Of the first 100 bursts, 26 will result in infection. Unfortunately, another feature of this evolved virus is speed; of the first 10000000 bursts, 2511944 will result in infection.

// Given your actual map, after 10000000 bursts of activity, how many bursts cause a node to become infected? (Do not count nodes that begin infected.)
extern crate itertools;

use std::collections::{HashMap,HashSet};
use std::fmt;
use self::itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum NodeState { Clean, Weakened, Infected, Flagged }

const MAP_GROW: usize = 2;

trait Map {
    fn node_stage(&mut self, coords:(isize, isize)) -> &mut usize;
    fn unclean_nodes(&self) -> HashMap<usize, HashSet<(isize, isize)>>;
}

struct AllocMap {
    size: usize,
    node_stages: Box<[usize]>
}

impl AllocMap {
    fn new(size: usize) -> Self {
        Self {
            size,
            node_stages: vec![0; size * size].into_boxed_slice()
        }
    }

    fn node_stage(&mut self, coords: (isize, isize)) -> &mut usize {
        let (x, y) = coords;

        let index = loop {
            let (_x, _y) = self.offset();

            if let Some(index) = Self::index((x + _x, y + _y), self.size) {
                break index
            }

            *self = self.grow();
        };

        &mut self.node_stages[index as usize]
    }

    fn index(coords: (isize, isize), size: usize) -> Option<usize> {
        let (x, y) = coords;

        if !(0..size as isize).contains(x) || !(0..size as isize).contains(y) {
            None
        } else {
            Some(y as usize * size + x as usize)
        }
    }

    fn _offset(size: usize) -> (isize, isize) {
        (size as isize / 2, size as isize / 2)
    }

    fn offset(&self) -> (isize, isize) {
        Self::_offset(self.size)
    }

    fn copy_node_stages(from: &Self, to: &mut Self) {
        let copy_start_offs = {
            let from_offs = from.offset();
            let to_offs = to.offset();
            (to_offs.0 - from_offs.0, to_offs.1 - from_offs.1)
        };
        let copy_start_ind = Self::index(copy_start_offs, to.size).unwrap();

        for i in 0..from.size {
            let from_row = {
                let start_ind = from.size * i;
                &from.node_stages[start_ind..start_ind + from.size]
            };
            let mut to_row = {
                let start_ind = copy_start_ind + to.size * i;
                &mut to.node_stages[start_ind..start_ind + from.size]
            };

            to_row.copy_from_slice(from_row);
        }
    }

    fn grow(&self) -> Self {
        let size = self.size * MAP_GROW;
        let mut new = Self::new(size);

        Self::copy_node_stages(&self, &mut new);

        new
    }
}

impl Map for AllocMap {
    fn node_stage(&mut self, coords: (isize, isize)) -> &mut usize {
        Self::node_stage(self, coords)
    }

    fn unclean_nodes(&self) -> HashMap<usize, HashSet<(isize, isize)>> {
        let unclean_nodes = self.node_stages.iter()
            .enumerate()
            .filter(|&(_i, &stage)| stage != 0)
            .map(|(i, &stage)| -> (usize, (isize, isize)) {
                let x = i % self.size;
                let y = i / self.size;
                let (_x, _y) = self.offset();
                (stage, (x as isize - _x, y as isize - _y))
            });

        let mut ret = HashMap::new();
        

        for (stage, coords) in unclean_nodes {
            let entry = ret.entry(stage).or_insert(HashSet::new());

            entry.insert(coords);
        }

        ret
    }
}

impl fmt::Debug for AllocMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let counts = {
            let mut counts_map: HashMap<usize, usize> = HashMap::new();

            for &node in self.node_stages.iter() {
                let mut num = counts_map.entry(node).or_insert(0);
                *num += 1;
            }

            counts_map.into_iter()
                .sorted().into_iter()
                .map(|(k, v)| format!("{:?}: {:?}", k, v))
                .join(", ")
        };

        writeln!(f, "Map size: {}, counts: {{ {} }}", self.size, counts)?;

        if self.size < 150 {
            for row in &self.node_stages.iter().chunks(self.size) {
                let row_repr: String = row.map(|&s| s.to_string()).collect();
                writeln!(f, "{}", row_repr)?;
            }
        }

        Ok(())
    }
}

impl Map for HashMap<(isize, isize), usize> {
    fn node_stage(&mut self, coords: (isize, isize)) -> &mut usize {
        self.entry(coords).or_insert(0)
    }

    fn unclean_nodes(&self) -> HashMap<usize, HashSet<(isize, isize)>> {
        let unclean_nodes = self.iter()
            .filter(|&(_coords, &stage)| stage != 0);
            

        let mut ret = HashMap::new();

        for (&coords, &stage) in unclean_nodes {
            let entry = ret.entry(stage).or_insert(HashSet::new());
            entry.insert(coords);
        }

        ret
    }
}

#[derive(Debug)]
struct Virus {
    pos: (isize, isize),
    vel: (isize, isize),
    infection_count: u32,
    node_transitions: Vec<NodeState>,
}

impl Virus {
    fn turn_left(&mut self) {
        let (x, y) = self.vel;
        self.vel = (-y, x);
    }

    fn turn_right(&mut self) {
        let (x, y) = self.vel;
        self.vel = (y, -x);
    }

    fn reverse(&mut self) {
        let (x, y) = self.vel;
        self.vel = (-x, -y);
    }

    fn burst(&mut self, map: &mut Map) {
        use self::NodeState::*;

        let node_stage = map.node_stage(self.pos);

        match self.node_transitions[*node_stage] {
            Clean => { self.turn_left(); },
            Weakened => {},
            Infected => { self.turn_right(); },
            Flagged => { self.reverse(); },
        }

        *node_stage = (*node_stage + 1) % self.node_transitions.len();

        if self.node_transitions[*node_stage] == Infected {
            self.infection_count += 1;
        }

        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }
}

fn solve(
    input: &str,
    map: &mut Map,
    node_transitions: Vec<NodeState>,
    burst_count: u32
) -> u32 {
    use self::NodeState::*;

    let x_max = input.split('\n').next().unwrap().len() as isize;
    let y_max = input.split('\n').count() as isize;
    let x_offs = x_max / 2;
    let y_offs = y_max / 2;

    let _state_stages: HashMap<NodeState, usize> = node_transitions.iter()
        .enumerate()
        .map(|(i, &state)| (state, i))
        .collect();
    
    let state_stages = &_state_stages;

    let map_start = input.split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let stage = match c {
                    '.' => state_stages[&Clean],
                    '#' => state_stages[&Infected],
                    _ => panic!()
                };

                let x_adjusted = x as isize - x_offs;
                // Invert y-coordinate since we read top-to-bottom
                let y_adjusted = y_max - 1 - y as isize - y_offs;

                ((x_adjusted, y_adjusted), stage)
            })
        });

    for (coords, stage) in map_start {
        *map.node_stage(coords) = stage;
    }

    let mut virus = Virus {
        pos: (0, 0),
        vel: (0, 1),
        infection_count: 0,
        node_transitions
    };

    for _ in 0..burst_count {
        virus.burst(map);
    }

    virus.infection_count
}

fn solve_alloc_map(
    input: &str,
    node_transitions: Vec<NodeState>,
    burst_count: u32
) -> u32 {
    let mut map = AllocMap::new(1);

    solve(input, &mut map, node_transitions, burst_count)
}

pub fn part1(input: &str) -> String {
    use self::NodeState::*;

    solve_alloc_map(input, vec![Clean, Infected], 10_000).to_string()
}

pub fn part2(input: &str) -> String {
    use self::NodeState::*;

    solve_alloc_map(input, vec![
        Clean,
        Weakened,
        Infected,
        Flagged
    ], 10_000_000).to_string()
}

#[test]
fn test_compare_maps() {
    fn solve_hash_map(
        input: &str,
        node_transitions: Vec<NodeState>,
        burst_count: u32
    ) -> u32 {
        let mut map = HashMap::new();

        solve(input, &mut map, node_transitions, burst_count)
    }

    use self::NodeState::*;

    let part1_transitions = || vec![Clean, Infected];

    let test_input ="..#
#..
...";

    for i in 0..1200 {
        let mut hash_map = HashMap::new();
        let mut alloc_map = AllocMap::new(1);

        solve(test_input, &mut hash_map, part1_transitions(), i);
        solve(test_input, &mut alloc_map, part1_transitions(), i);

        let hu = hash_map.unclean_nodes();
        let au = alloc_map.unclean_nodes();

        assert_eq!(
            hash_map.unclean_nodes(),
            alloc_map.unclean_nodes(),
            "i == {}\nin hash: {:?}\nin alloc: {:?}",
            i,
            &hu[&1] - &au[&1],
            &au[&1] - &hu[&1],
        );
    }
}

#[test]
fn test_sample_input() {
    use self::NodeState::*;

    let part1_transitions = || vec![Clean, Infected];
    let part2_transitions = || vec![Clean, Weakened, Infected, Flagged];

    let test_input ="..#
#..
...";

    assert_eq!(solve_alloc_map(test_input, part1_transitions(), 7), 5);
    assert_eq!(solve_alloc_map(test_input, part1_transitions(), 70), 41);
    assert_eq!(solve_alloc_map(test_input, part1_transitions(), 10000), 5587);
    assert_eq!(solve_alloc_map(test_input, part2_transitions(), 100), 26);
    assert_eq!(solve_alloc_map(test_input, part2_transitions(), 10000000), 2511944);
}