// --- Day 18: Duet ---

// You discover a tablet containing some strange assembly code labeled simply "Duet". Rather than bother the sound card with it, you decide to run the code yourself. Unfortunately, you don't see any documentation, so you're left to figure out what the instructions mean on your own.

// It seems like the assembly is meant to operate on a set of registers that are each named with a single letter and that can each hold a single integer. You suppose each register should start with a value of 0.

// There aren't that many instructions, so it shouldn't be hard to figure out what they do. Here's what you determine:

//     snd X plays a sound with a frequency equal to the value of X.
//     set X Y sets register X to the value of Y.
//     add X Y increases register X by the value of Y.
//     mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
//     mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
//     rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
//     jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)

// Many of the instructions can take either a register (a single letter) or a number. The value of a register is the integer it contains; the value of a number is that number.

// After each jump instruction, the program continues with the instruction to which the jump jumped. After any other instruction, the program continues with the next instruction. Continuing (or jumping) off either end of the program terminates it.

// For example:

// set a 1
// add a 2
// mul a a
// mod a 5
// snd a
// set a 0
// rcv a
// jgz a -1
// set a 1
// jgz a -2

//     The first four instructions set a to 1, add 2 to it, square it, and then set it to itself modulo 5, resulting in a value of 4.
//     Then, a sound with frequency 4 (the value of a) is played.
//     After that, a is set to 0, causing the subsequent rcv and jgz instructions to both be skipped (rcv because a is 0, and jgz because a is not greater than 0).
//     Finally, a is set to 1, causing the next jgz instruction to activate, jumping back two instructions to another jump, which jumps again to the rcv, which ultimately triggers the recover operation.

// At the time the recover operation is executed, the frequency of the last sound played is 4.

// What is the value of the recovered frequency (the value of the most recently played sound) the first time a rcv instruction is executed with a non-zero value?
use std::collections::HashMap;
use std::str::FromStr;

struct Duet {
    instructions: Vec<Instruction>,
    current_instr: isize,
    registers: HashMap<char, isize>,
    last_tone: Option<isize>,
    last_recovered_tone: Option<isize>,
}

impl Duet {
    fn new(instructions: Vec<Instruction>) -> Self {
        Duet {
            instructions,
            current_instr: 0,
            registers: HashMap::new(),
            last_tone: None,
            last_recovered_tone: None,
        }
    }

    fn run_instruction(&mut self) {
        use self::Instruction::*;

        let mut next_jump = 1;

        match self.instructions[self.current_instr as usize] {
            Snd(reg) => { self.last_tone = Some(*self.index(reg)); },
            Set(reg, arg) => { *self.index(reg) = self.get(arg); },
            Add(reg, arg) => { *self.index(reg) += self.get(arg); },
            Mul(reg, arg) => { *self.index(reg) *= self.get(arg); }
            Mod(reg, arg) => { *self.index(reg) %= self.get(arg); },
            Rcv(reg) => {
                if *self.index(reg) != 0 {
                    self.last_recovered_tone = self.last_tone;
                }
            }
            Jgz(reg, arg) => {
                if *self.index(reg) > 0 {
                    next_jump = self.get(arg);
                }
            }
        }

        self.current_instr += next_jump;
    }

    fn index(&mut self, reg: char) -> &mut isize {
        self.registers.entry(reg).or_insert(0)
    }

    fn get(&mut self, arg: InstructionArg) -> isize {
        match arg {
            InstructionArg::Val(val) => val,
            InstructionArg::Reg(reg) => *self.index(reg)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(char),
    Set(char, InstructionArg),
    Add(char, InstructionArg),
    Mul(char, InstructionArg),
    Mod(char, InstructionArg),
    Rcv(char),
    Jgz(char, InstructionArg),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Instruction::*;

        let mut toks = s.split_whitespace();
        let instr = toks.next().unwrap();
        let reg: char = toks.next().unwrap().parse().unwrap();
        let arg = toks.next();

        Ok(match (instr, reg, arg) {
            ("snd", reg, None) => Snd(reg),
            ("set", reg, Some(arg)) => Set(reg, arg.parse().unwrap()),
            ("add", reg, Some(arg)) => Add(reg, arg.parse().unwrap()),
            ("mul", reg, Some(arg)) => Mul(reg, arg.parse().unwrap()),
            ("mod", reg, Some(arg)) => Mod(reg, arg.parse().unwrap()),
            ("rcv", reg, None) => Rcv(reg),
            ("jgz", reg, Some(arg)) => Jgz(reg, arg.parse().unwrap()),
            (i, r, a) => panic!("Invalid instruction '{:?}'", (i, r, a))
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum InstructionArg {
    Reg(char),
    Val(isize),
}

impl FromStr for InstructionArg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::InstructionArg::*;

        Ok(match s.chars().next() {
            Some(reg @ 'a'...'p') => Reg(reg),
            Some('-') | Some('0'...'9') => Val(s.parse().unwrap()),
            _ => panic!("Invalid instruction argument '{}'", s)
        })
    }
}

pub fn part1(input: &str) -> String {
    let instructions = input.split('\n').map(|s| s.parse().unwrap()).collect();
    let mut duet = Duet::new(instructions);

    while duet.last_recovered_tone == None {
        duet.run_instruction();
    };

    duet.last_recovered_tone.unwrap().to_string()
}