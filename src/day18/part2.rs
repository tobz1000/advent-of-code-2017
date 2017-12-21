// --- Part Two ---

// As you congratulate yourself for a job well done, you notice that the documentation has been on the back of the tablet this entire time. While you actually got most of the instructions correct, there are a few key differences. This assembly code isn't about sound at all - it's meant to be run twice at the same time.

// Each running copy of the program has its own set of registers and follows the code independently - in fact, the programs don't even necessarily run at the same speed. To coordinate, they use the send (snd) and receive (rcv) instructions:

//     snd X sends the value of X to the other program. These values wait in a queue until that program is ready to receive them. Each program has its own message queue, so a program can never receive a message it sent.
//     rcv X receives the next value and stores it in register X. If no values are in the queue, the program waits for a value to be sent to it. Programs do not continue to the next instruction until they have received a value. Values are received in the order they are sent.

// Each program also has its own program ID (one 0 and the other 1); the register p should begin with this value.

// For example:

// snd 1
// snd 2
// snd p
// rcv a
// rcv b
// rcv c
// rcv d

// Both programs begin by sending three values to the other. Program 0 sends 1, 2, 0; program 1 sends 1, 2, 1. Then, each program receives a value (both 1) and stores it in a, receives another value (both 2) and stores it in b, and then each receives the program ID of the other program (program 0 receives 1; program 1 receives 0) and stores it in c. Each program now sees a different value in its own copy of register c.

// Finally, both programs try to rcv a fourth time, but no data is waiting for either of them, and they reach a deadlock. When this happens, both programs terminate.

// It should be noted that it would be equally valid for the programs to run at different speeds; for example, program 0 might have sent all three values and then stopped at the first rcv before program 1 executed even its first instruction.

// Once both of your programs have terminated (regardless of what caused them to do so), how many times did program 1 send a value?
use std::collections::{HashMap,VecDeque};
use super::{Instruction, InstructionArg};

#[derive(Debug, PartialEq, Clone, Copy)]
enum DuetState { Continue, Wait, Terminate }

#[derive(Debug)]
struct Duet<'a> {
    instructions: &'a Vec<Instruction>,
    reg_init: isize,
    current_instr: isize,
    registers: HashMap<char, isize>,
    send_count: usize,
    state: DuetState,
}

impl<'a> Duet<'a> {
    fn new(instructions: &'a Vec<Instruction>, reg_init: isize) -> Self {
        Duet {
            instructions,
            reg_init,
            current_instr: 0,
            registers: HashMap::new(),
            send_count: 0,
            state: DuetState::Continue,
        }
    }

    fn run(
        &mut self,
        rcv_q: &mut VecDeque<isize>,
        send_q: &mut VecDeque<isize>,
    ) {
        use self::DuetState::*;

        if self.state == Terminate {
            return;
        }

        loop {
            self.run_instruction(rcv_q, send_q);

            match self.state {
                Continue => { continue; },
                Wait | Terminate => { break; },
            }
        }
    }

    fn run_instruction(
        &mut self,
        rcv_q: &mut VecDeque<isize>,
        send_q: &mut VecDeque<isize>,
    ) {
        use self::Instruction::*;
        use self::DuetState::*;

        let mut next_jump = 1;
        let mut state = Continue;

        match self.instructions[self.current_instr as usize] {
            Snd(reg) => {
                send_q.push_back(*self.index(reg));
                self.send_count += 1;
            },
            Set(reg, arg) => { *self.index(reg) = self.get(arg); },
            Add(reg, arg) => { *self.index(reg) += self.get(arg); },
            Mul(reg, arg) => { *self.index(reg) *= self.get(arg); }
            Mod(reg, arg) => { *self.index(reg) %= self.get(arg); },
            Rcv(reg) => {
                match rcv_q.pop_front() {
                    Some(val) => { *self.index(reg) = val; },
                    None => {   
                        next_jump = 0;

                        state = match self.state {
                            Continue => Wait,
                            Wait => Terminate,
                            Terminate => panic!()
                        };
                    }
                }
            },
            Jgz(check, jump) => {
                if self.get(check) > 0 {
                    next_jump = self.get(jump);
                }
            }
        }

        self.current_instr += next_jump;

        if !(0..self.instructions.len() as isize).contains(self.current_instr) {
            state = Terminate;
        }

        self.state = state;
    }

    fn index(&mut self, reg: char) -> &mut isize {
        self.registers.entry(reg).or_insert(self.reg_init)
    }

    fn get(&mut self, arg: InstructionArg) -> isize {
        match arg {
            InstructionArg::Val(val) => val,
            InstructionArg::Reg(reg) => *self.index(reg)
        }
    }
}

pub fn part2(input: &str) -> String {
    use self::DuetState::*;

    let instructions = input.split('\n').map(|s| s.parse().unwrap()).collect();

    let mut zero_rcv_q = VecDeque::new();
    let mut one_rcv_q = VecDeque::new();

    let mut zero = Duet::new(&instructions, 0);
    let mut one = Duet::new(&instructions, 1);

    while zero.state != Terminate && one.state != Terminate {
        zero.run(&mut zero_rcv_q, &mut one_rcv_q);
        one.run(&mut one_rcv_q, &mut zero_rcv_q);
    }

    let ans = one.send_count;
    ans.to_string()
}