use anyhow::anyhow as e;
use std::fmt::Display;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Machine {
    x: isize,
}

impl Machine {
    fn new() -> Self {
        Self { x: 1 }
    }

    fn execute(&mut self, op: &Op) {
        match op {
            Op::Noop => (),
            Op::Addx(n) => {
                self.x += n;
            }
        }
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
    history: Vec<ProgramHistory>,
}

#[derive(Debug, Clone, Copy)]
struct ProgramHistory {
    initial_state: Machine,
    op: Op,
    elapsed_cycles: usize,
}

impl Program {
    fn new() -> Self {
        Self {
            machine: Machine::new(),
            history: Vec::new(),
        }
    }

    fn state_at_cycle(&self, cycle: usize) -> Option<Machine> {
        assert_ne!(cycle, 220);
        let mut prev = None;
        for entry in &self.history {
            match cycle.cmp(&entry.elapsed_cycles) {
                std::cmp::Ordering::Less => return prev,
                std::cmp::Ordering::Equal => return Some(entry.initial_state),
                std::cmp::Ordering::Greater => prev = Some(entry.initial_state),
            }
        }
        None
    }

    fn signal_strength(&self, cycle: usize) -> Option<isize> {
        self.state_at_cycle(cycle - 1)
            .map(|state| state.x * (cycle as isize))
    }

    fn run(&mut self, ops: &[Op]) {
        let mut elapsed_cycles = 0;
        for op in ops {
            self.history.push(ProgramHistory {
                initial_state: self.machine,
                op: *op,
                elapsed_cycles,
            });
            self.machine.execute(op);
            elapsed_cycles += op.cycles();
        }
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
    let checks: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let total: isize = checks
        .iter()
        .map(|cycle| program.signal_strength(*cycle).unwrap())
        .sum();
    println!("{total}");
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
        let expected: Vec<isize> = vec![1, 16, 5, 11, 8, 13, 12, 4, 17, 21];
        for (history, expected_x) in program.history.iter().zip(expected) {
            assert_eq!(history.initial_state.x, expected_x);
        }
        dbg!(&program.history);

        assert_eq!(program.signal_strength(20), Some(420));
        assert_eq!(program.signal_strength(60), Some(1140));
        assert_eq!(program.signal_strength(100), Some(1800));
        assert_eq!(program.signal_strength(140), Some(2940));
        assert_eq!(program.signal_strength(180), Some(2880));
        assert_eq!(program.signal_strength(220), Some(3960));

        Ok(())
    }
}
