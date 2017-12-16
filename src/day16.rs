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