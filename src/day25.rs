// --- Day 25: The Halting Problem ---

// Following the twisty passageways deeper and deeper into the CPU, you finally reach the core of the computer. Here, in the expansive central chamber, you find a grand apparatus that fills the entire room, suspended nanometers above your head.

// You had always imagined CPUs to be noisy, chaotic places, bustling with activity. Instead, the room is quiet, motionless, and dark.

// Suddenly, you and the CPU's garbage collector startle each other. "It's not often we get many visitors here!", he says. You inquire about the stopped machinery.

// "It stopped milliseconds ago; not sure why. I'm a garbage collector, not a doctor." You ask what the machine is for.

// "Programs these days, don't know their origins. That's the Turing machine! It's what makes the whole computer work." You try to explain that Turing machines are merely models of computation, but he cuts you off. "No, see, that's just what they want you to think. Ultimately, inside every CPU, there's a Turing machine driving the whole thing! Too bad this one's broken. We're doomed!"

// You ask how you can help. "Well, unfortunately, the only way to get the computer running again would be to create a whole new Turing machine from scratch, but there's no way you can-" He notices the look on your face, gives you a curious glance, shrugs, and goes back to sweeping the floor.

// You find the Turing machine blueprints (your puzzle input) on a tablet in a nearby pile of debris. Looking back up at the broken Turing machine above, you can start to identify its parts:

//     A tape which contains 0 repeated infinitely to the left and right.
//     A cursor, which can move left or right along the tape and read or write values at its current position.
//     A set of states, each containing rules about what to do based on the current value under the cursor.

// Each slot on the tape has two possible values: 0 (the starting value for all slots) and 1. Based on whether the cursor is pointing at a 0 or a 1, the current state says what value to write at the current position of the cursor, whether to move the cursor left or right one slot, and which state to use next.

// For example, suppose you found the following blueprint:

// Begin in state A.
// Perform a diagnostic checksum after 6 steps.

// In state A:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state B.
//   If the current value is 1:
//     - Write the value 0.
//     - Move one slot to the left.
//     - Continue with state B.

// In state B:
//   If the current value is 0:
//     - Write the value 1.
//     - Move one slot to the left.
//     - Continue with state A.
//   If the current value is 1:
//     - Write the value 1.
//     - Move one slot to the right.
//     - Continue with state A.

// Running it until the number of steps required to take the listed diagnostic checksum would result in the following tape configurations (with the cursor marked in square brackets):

// ... 0  0  0 [0] 0  0 ... (before any steps; about to run state A)
// ... 0  0  0  1 [0] 0 ... (after 1 step;     about to run state B)
// ... 0  0  0 [1] 1  0 ... (after 2 steps;    about to run state A)
// ... 0  0 [0] 0  1  0 ... (after 3 steps;    about to run state B)
// ... 0 [0] 1  0  1  0 ... (after 4 steps;    about to run state A)
// ... 0  1 [1] 0  1  0 ... (after 5 steps;    about to run state B)
// ... 0  1  1 [0] 1  0 ... (after 6 steps;    about to run state A)

// The CPU can confirm that the Turing machine is working by taking a diagnostic checksum after a specific number of steps (given in the blueprint). Once the specified number of steps have been executed, the Turing machine should pause; once it does, count the number of times 1 appears on the tape. In the above example, the diagnostic checksum is 3.

// Recreate the Turing machine and save the computer! What is the diagnostic checksum it produces once it's working again?
extern crate regex;

use self::regex::Regex;

struct Machine {
    tape: Tape,
    states: Vec<[Action; 2]>,
    curr_state: u8,
    pos: isize
}

impl Machine {
    fn new(states: Vec<[Action; 2]>, start_state: u8) -> Self {
        Machine {
            tape: Tape::new(16),
            states,
            curr_state: start_state,
            pos: 0
        }
    }

    fn run(&mut self) {
        let pos = self.pos;
        let curr_state = self.curr_state;
        let slot = self.tape.val(pos);
        let Action {
            write,
            dir,
            next_state
        } = self.states[curr_state as usize][*slot as usize];
        
        *slot = write;
        self.pos += dir;
        self.curr_state = next_state;
    }
}

const TAPE_GROW: usize = 2;

struct Tape {
    states: Vec<u8>,
}

impl Tape {
    fn new(size: usize) -> Self {
        Tape {
            states: vec![0; size],
        }
    }

    fn val(&mut self, slot: isize) -> &mut u8 {
        let index = loop {
            let index = slot + self.offset() as isize;

            if (0..self.states.len() as isize).contains(index) {
                break index as usize;
            }

            *self = self.grow();
        };

        &mut self.states[index]
    }

    fn offset(&self) -> usize {
        self.states.len() / 2
    }

    fn copy_vals(from: &Self, to: &mut Self) {
        let copy_start_offs = to.offset() - from.offset();
        let dest = &mut to.states[
            copy_start_offs..copy_start_offs + from.states.len()
        ];

        dest.copy_from_slice(&from.states);
    }

    fn grow(&self) -> Self {
        let mut new = Tape::new(self.states.len() * TAPE_GROW);

        Self::copy_vals(self, &mut new);

        new
    }
}

#[derive(Debug, Copy, Clone)]
struct Action {
    write: u8,
    dir: isize,
    next_state: u8,
}

impl Action {
    fn from_words(write_str: &str, dir_str: &str, next_str: &str) -> Self {
        let write = match write_str {
            "0" => 0,
            "1" => 1,
            _ => panic!()
        };
        let dir = match dir_str {
            "left" => -1,
            "right" => 1,
            _ => panic!()
        };
        let next_state = state_index(next_str);

        Action { write, dir, next_state }
    }
}

fn state_index(label: &str) -> u8 {
    label.chars().next().unwrap() as u8 - 'A' as u8
}

// #[derive(Debug)]
// struct State {
//     action0: Action,
//     action1: Action,
// }

pub fn part1(input: &str) -> String {
    let intro_reg = Regex::new(
r"Begin in state (?P<startstate>[A-Z]).
Perform a diagnostic checksum after (?P<stepcount>\d*) steps."
    ).unwrap();

    let state_spec_reg = Regex::new(
r"In state (?P<statename>[A-Z]):
  If the current value is 0:
    - Write the value (?P<write0>[01]).
    - Move one slot to the (?P<dir0>left|right).
    - Continue with state (?P<next0>[A-Z]).
  If the current value is 1:
    - Write the value (?P<write1>[01]).
    - Move one slot to the (?P<dir1>left|right).
    - Continue with state (?P<next1>[A-Z])."
    ).unwrap();

    let intro_caps = intro_reg.captures(input).unwrap();

    let start_state = state_index(&intro_caps["startstate"]);
    let step_count = intro_caps["stepcount"].parse().unwrap();

    let states = state_spec_reg.captures_iter(input).map(|caps| {
        let action0 = Action::from_words(
            &caps["write0"],
            &caps["dir0"],
            &caps["next0"]
        );
        let action1 = Action::from_words(
            &caps["write1"],
            &caps["dir1"],
            &caps["next1"]
        );

        [action0, action1]
    }).collect();

    // for s in states {
    //     println!("{:?}", s);
    // }

    let mut machine = Machine::new(states, start_state);

    for _ in 0..step_count {
        machine.run();
    }

    let ans = machine.tape.states.iter().filter(|&&s| s == 1).count();

    ans.to_string()
}