// --- Day 23: Coprocessor Conflagration ---

// You decide to head directly to the CPU and fix the printer from there. As you get close, you find an experimental coprocessor doing so much work that the local programs are afraid it will halt and catch fire. This would cause serious issues for the rest of the computer, so you head in and see what you can do.

// The code it's running seems to be a variant of the kind you saw recently on that tablet. The general functionality seems very similar, but some of the instructions are different:

//     set X Y sets register X to the value of Y.
//     sub X Y decreases register X by the value of Y.
//     mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
//     jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)

//     Only the instructions listed above are used. The eight registers here, named a through h, all start at 0.

// The coprocessor is currently set to some kind of debug mode, which allows for testing, but prevents it from doing any meaningful work.

// If you run the program (your puzzle input), how many times is the mul instruction invoked?
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Set(char, InstructionArg),
    Sub(char, InstructionArg),
    Mul(char, InstructionArg),
    Jnz(InstructionArg, InstructionArg),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Instruction::*;
        use self::InstructionArg::*;

        let mut toks = s.split_whitespace();
        let instr = toks.next().unwrap();
        let arg1: InstructionArg = toks.next().unwrap().parse().unwrap();
        let arg2: InstructionArg = toks.next().unwrap().parse().unwrap();

        Ok(match (instr, arg1, arg2) {
            ("set", Reg(reg), arg) => Set(reg, arg),
            ("sub", Reg(reg), arg) => Sub(reg, arg),
            ("mul", Reg(reg), arg) => Mul(reg, arg),
            ("jnz", arg1, arg2) => Jnz(arg1, arg2),
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

struct Processor {
    instructions: Vec<Instruction>,
    current_instr: isize,
    registers: HashMap<char, isize>,
    mul_count: usize
}

impl Processor {
    fn new(instructions: Vec<Instruction>) -> Self {
        Processor {
            instructions,
            current_instr: 0,
            registers: HashMap::new(),
            mul_count: 0,
        }
    }

    fn run_instruction(&mut self) {
        use self::Instruction::*;

        let mut next_jump = 1;

        match self.instructions[self.current_instr as usize] {
            Set(reg, arg) => { *self.index(reg) = self.get(arg); },
            Sub(reg, arg) => { *self.index(reg) -= self.get(arg); },
            Mul(reg, arg) => {
                self.mul_count += 1;
                *self.index(reg) *= self.get(arg);
            },
            Jnz(check, jump) => {
                if self.get(check) != 0 {
                    next_jump = self.get(jump);
                }
            }
        }

        self.current_instr += next_jump;
    }

    fn index(&mut self, reg: char) -> &mut isize {
        self.registers.entry(reg).or_insert(0)
    }

    fn running(&self) -> bool {
        (0..self.instructions.len() as isize).contains(self.current_instr)
    }

    fn get(&mut self, arg: InstructionArg) -> isize {
        match arg {
            InstructionArg::Val(val) => val,
            InstructionArg::Reg(reg) => *self.index(reg)
        }
    }
}

pub fn part1(input: &str) -> String {
    let instructions = input.split('\n').map(|s| s.parse().unwrap()).collect();
    let mut processor = Processor::new(instructions);

    while processor.running() {
        processor.run_instruction();
    };

    processor.mul_count.to_string()
}

// --- Part Two ---

// Now, it's time to fix the problem.

// The debug mode switch is wired directly to register a. You flip the switch, which makes register a now start at 1 when the program is executed.

// Immediately, the coprocessor begins to overheat. Whoever wrote this program obviously didn't choose a very efficient implementation. You'll need to optimize the program if it has any hope of completing before Santa needs that printer working.

// The coprocessor's ultimate goal is to determine the final value left in register h once the program completes. Technically, if it had that... it wouldn't even need to run the program.

// After setting register a to 1, if the program were to run to completion, what value would be left in register h?
