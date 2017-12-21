mod part1;
mod part2;

pub use self::part1::part1;
pub use self::part2::part2;

use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Snd(char),
    Set(char, InstructionArg),
    Add(char, InstructionArg),
    Mul(char, InstructionArg),
    Mod(char, InstructionArg),
    Rcv(char),
    Jgz(InstructionArg, InstructionArg),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Instruction::*;
        use self::InstructionArg::*;

        let mut toks = s.split_whitespace();
        let instr = toks.next().unwrap();
        let arg1: InstructionArg = toks.next().unwrap().parse().unwrap();
        let arg2: Option<InstructionArg> = toks.next()
            .and_then(|arg| Some(arg.parse().unwrap()));

        Ok(match (instr, arg1, arg2) {
            ("snd", Reg(reg), None) => Snd(reg),
            ("set", Reg(reg), Some(arg)) => Set(reg, arg),
            ("add", Reg(reg), Some(arg)) => Add(reg, arg),
            ("mul", Reg(reg), Some(arg)) => Mul(reg, arg),
            ("mod", Reg(reg), Some(arg)) => Mod(reg, arg),
            ("rcv", Reg(reg), None) => Rcv(reg),
            ("jgz", arg1, Some(arg2)) => Jgz(arg1, arg2),
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