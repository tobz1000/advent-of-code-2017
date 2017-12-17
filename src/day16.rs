// --- Day 16: Permutation Promenade ---

// You come upon a very unusual sight; a group of programs here appear to be dancing.

// There are sixteen programs in total, named a through p. They start by standing in a line: a stands in position 0, b stands in position 1, and so on until p, which stands in position 15.

// The programs' dance consists of a sequence of dance moves:

//     Spin, written sX, makes X programs move from the end to the front, but maintain their order otherwise. (For example, s3 on abcde produces cdeab).
//     Exchange, written xA/B, makes the programs at positions A and B swap places.
//     Partner, written pA/B, makes the programs named A and B swap places.

// For example, with only five programs standing in a line (abcde), they could do the following dance:

//     s1, a spin of size 1: eabcd.
//     x3/4, swapping the last two programs: eabdc.
//     pe/b, swapping programs e and b: baedc.

// After finishing their dance, the programs end up in order baedc.

// You watch the dance for a while and record their dance moves (your puzzle input). In what order are the programs standing after their dance?
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
struct Programs {
    state: VecDeque<char>
}

impl Programs {
    fn new() -> Self {
        let state: VecDeque<char> = (0..16).map(|i| (('a' as u8) + i) as char).collect();

        Programs { state }
    }

    fn action(&mut self, action: Action) {
        use self::Action::*;
        match action {
            Spin(x) => {
                for _ in 0..x {
                    let elm = self.state.pop_back().unwrap();
                    self.state.push_front(elm);
                }
            },
            Exchange(n, m) => {
                self.state.swap(n, m);
            },
            Partner(a, b) => {
                let (a_pos, b_pos) = {
                    let (a_pos, _a) = self.state.iter()
                        .enumerate()
                        .find(|&(_i, &c)| c == a)
                        .unwrap();

                    let (b_pos, _b) = self.state.iter()
                        .enumerate()
                        .find(|&(_i, &c)| c == b)
                        .unwrap();
                    
                    (a_pos, b_pos)
                };

                self.state.swap(a_pos, b_pos);
            }
        }
    }
}

enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Action::*;

        let (action, params) = s.split_at(1);

        match action {
            "s" => {
                Ok(Spin(params.parse().unwrap()))
            },
            "x" => {
                let mut indices = params.split('/');
                let first = indices.next().unwrap().parse().unwrap();
                let second = indices.next().unwrap().parse().unwrap();
                Ok(Exchange(first, second))
            },
            "p" => {
                let mut indices = params.split('/');
                let first = indices.next().unwrap().parse().unwrap();
                let second = indices.next().unwrap().parse().unwrap();
                Ok(Partner(first, second))
            },
            _ => panic!()
        }
    }
}

pub fn part1(input: &str) -> String {
    let mut programs = Programs::new();

    for action_str in input.split(',') {
        programs.action(action_str.parse().unwrap());
    }

    programs.state.iter().collect()
}

// --- Part Two ---

// Now that you're starting to get a feel for the dance moves, you turn your attention to the dance as a whole.

// Keeping the positions they ended up in from their previous dance, the programs perform it again and again: including the first dance, a total of one billion (1000000000) times.

// In the example above, their second dance would begin with the order baedc, and use the same dance moves:

//     s1, a spin o size 1: cbaed.
//     x3/4, swapping the last two programs: cbade.
//     pe/b, swapping programs e and b: ceadb.

// In what order are the programs standing after their billion dances?
struct Positions {
    state: VecDeque<u8>
}

impl Positions {
    fn new() -> Self {
        Positions { state: (0..16).collect() }
    }

    fn from_actions<I: Iterator<Item=Action>>(actions: I) -> Self {
        let mut positions = Positions::new();

        for action in actions {
            positions.action(&action);
        }

        positions
    } 

    fn action(&mut self, action: &Action) {
        use self::Action::*;
        match *action {
            Spin(x) => {
                for _ in 0..x {
                    let elm = self.state.pop_back().unwrap();
                    self.state.push_front(elm);
                }
            },
            Exchange(n, m) => {
                self.state.swap(n, m);
            },
            Partner(a, b) => {
                let num_a = a as u8 - 'a' as u8;
                let num_b = b as u8 - 'a' as u8;
                let pos_a = self.state.iter().position(|&x| x == num_a).unwrap();
                let pos_b = self.state.iter().position(|&x| x == num_b).unwrap();
                self.state.swap(pos_a, pos_b);
            }
        }
    }

    fn cycle(&self, pos: usize) -> Vec<u8> {
        let mut cycle = Vec::new();
        let mut cycle_next = pos as u8;

        loop {
            cycle.push(cycle_next);
            cycle_next = self.state[cycle_next as usize];
            if cycle_next == pos as u8 {
                break;
            }
        }

        cycle
    }

    fn repeat_permutation(&self, count: usize) -> Self {
        let state = (0..self.state.len()).map(|i| {
            let cycle = self.cycle(i);
            cycle[count % cycle.len()]
        }).collect();

        Positions { state }
    }
}

pub fn part2(input: &str) -> String {
    use self::Action::*;
    let repeat_count = 1_000_000_000;

    let (positional_actions, value_actions): (Vec<Action>, Vec<Action>) = input.split(',')
        .map(|s| s.parse().unwrap())
        .partition(|action| match *action {
            Spin(..) | Exchange(..) => true,
            Partner(..) => false
        });

    let positional_state = Positions::from_actions(positional_actions.into_iter())
        .repeat_permutation(repeat_count);
    let value_state = Positions::from_actions(value_actions.into_iter())
        .repeat_permutation(repeat_count);

    let ans = positional_state.state.iter()
        .map(|&n| (value_state.state[n as usize] + 'a' as u8) as char)
        .collect();
    
    ans
}