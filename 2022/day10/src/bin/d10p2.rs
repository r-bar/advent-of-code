use anyhow::anyhow as e;
use std::fmt::{Display, Write};
use std::str::FromStr;

const SCREEN_WIDTH: usize = 40;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Machine {
    x: isize,
    cycle: usize,
}

impl Machine {
    fn new() -> Self {
        Self { x: 1, cycle: 0 }
    }

    fn execute(&mut self, op: &Op) -> Vec<bool> {
        let mut pixels = Vec::new();
        for _ in 0..op.cycles() {
            pixels.push(self.tick());
        }
        match op {
            Op::Noop => (),
            Op::Addx(n) => {
                self.x += n;
            }
        }
        pixels
    }

    fn tick(&mut self) -> bool {
        let drawing = (self.cycle % SCREEN_WIDTH) as isize;
        self.cycle += 1;
        [self.x - 1, self.x, self.x + 1].contains(&drawing)
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Machine {{ x: {} }}", self.x)
    }
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Op {
    Noop,
    Addx(isize),
}

impl Op {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }

    fn parse_all(s: &str) -> anyhow::Result<Vec<Self>> {
        s.lines().map(|line| line.parse()).collect()
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end();
        let parts: Vec<_> = s.split_whitespace().collect();
        match parts.as_slice() {
            ["noop"] => Ok(Self::Noop),
            ["addx", n] => Ok(Self::Addx(n.parse()?)),
            _ => Err(e!("invalid command: {s}")),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Noop => f.write_str("noop"),
            Self::Addx(n) => write!(f, "addx {n}"),
        }
    }
}

impl std::fmt::Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

struct Program {
    machine: Machine,
    screen: Vec<bool>,
}

impl Program {
    fn new() -> Self {
        Self {
            machine: Machine::new(),
            screen: Vec::new(),
        }
    }

    fn run(&mut self, ops: &[Op]) {
        for op in ops {
            let pixels = self.machine.execute(op);
            self.screen.extend(pixels);
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, pixel) in self.screen.iter().enumerate() {
            f.write_char(if *pixel { '#' } else { ' ' })?;
            if i + 1 < self.screen.len() && (i + 1) % 40 == 0 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let asm = std::fs::read_to_string(&filename)?;
    let mut program = Program::new();
    let ops = Op::parse_all(&asm)?;
    program.run(&ops);
    println!("{program}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let asm = include_str!("../../example.txt");
        let ops = Op::parse_all(asm)?;
        let mut program = Program::new();
        program.run(&ops);
        let expected = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n\
                        ###   ###   ###   ###   ###   ###   ### \n\
                        ####    ####    ####    ####    ####    \n\
                        #####     #####     #####     #####     \n\
                        ######      ######      ######      ####\n\
                        #######       #######       #######     ";
        println!("program:\n{program}\n\nexpected:\n{expected}");
        assert_eq!(format!("{program}"), expected, "outputs do not match");
        Ok(())
    }
}
