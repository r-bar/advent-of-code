use anyhow::anyhow as e;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

lazy_static! {
    static ref STACK_RE: Regex = Regex::new(r"\[([A-Z])\]").unwrap();
    static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    static ref COLUMN_RE: Regex = Regex::new(r"\d+").unwrap();
}

struct Stacks {
    columns: Vec<Vec<char>>,
}

impl Stacks {
    fn new(cols: usize) -> Self {
        let mut columns = Vec::with_capacity(cols);
        for _ in 0..cols {
            columns.push(Vec::new())
        }
        Self { columns }
    }

    fn push(&mut self, col: usize, value: char) -> anyhow::Result<()> {
        self.columns
            .get_mut(col - 1)
            .ok_or_else(|| e!("invalid column {}", col))?
            .push(value);
        Ok(())
    }

    // Move a crate off the given stack
    fn pop(&mut self, col: usize) -> Option<char> {
        self.columns.get_mut(col - 1)?.pop()
    }

    fn popn(&mut self, col: usize, n: usize) -> anyhow::Result<Vec<char>> {
        let mut out = Vec::new();
        for _ in 0..n {
            out.push(
                self.pop(col)
                    .ok_or_else(|| e!("not enough crates in col {}", col))?,
            );
        }
        out.reverse();
        Ok(out)
    }

    // the reference to a vec of chars is because [char] does not have a size at compile time
    #[allow(clippy::ptr_arg)]
    fn pushn(&mut self, col: usize, vals: &Vec<char>) -> anyhow::Result<()> {
        self.columns
            .get_mut(col - 1)
            .ok_or_else(|| e!("invalid column {}", col))?
            .extend_from_slice(vals);
        Ok(())
    }

    // Move count crates from one stack to another
    fn mv(&mut self, count: usize, from_col: usize, to_col: usize) -> anyhow::Result<()> {
        //let crates =
        let vals = self.popn(from_col, count)?;
        self.pushn(to_col, &vals)?;
        Ok(())
    }

    /// List the top crate in each stack if there is one
    fn list_tops(&self) -> Vec<Option<char>> {
        self.columns.iter().map(|col| col.last().copied()).collect()
    }

    /// See the top crate of the given stack number
    fn top(&self, col: usize) -> Option<char> {
        self.columns.get(col).and_then(|col| col.last().copied())
    }
}

struct InputColumns {
    positions: Vec<usize>,
}

impl InputColumns {
    fn len(&self) -> usize {
        self.positions.len()
    }

    /// Returns a list of columns and characters to
    fn parse_stack_line(&self, line: &str) -> Vec<(usize, char)> {
        let mut output = Vec::new();
        for (start, col) in self.positions.iter().zip(1..) {
            let c = line.chars().nth(*start).unwrap_or(' ');
            if c.is_alphabetic() {
                output.push((col, c))
            }
        }
        output
    }
}

impl FromStr for InputColumns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions: Vec<usize> = Vec::new();
        for m in COLUMN_RE.find_iter(s) {
            positions.push(m.start())
        }
        Ok(Self { positions })
    }
}

fn parse_stacks(reader: &mut BufReader<File>) -> anyhow::Result<Stacks> {
    let mut puzzle_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?.trim_end().to_string();
        if line.is_empty() {
            break;
        }
        puzzle_lines.push(line)
    }
    let mut puzzle_iter = puzzle_lines.into_iter().rev();
    let column_line = puzzle_iter
        .next()
        .ok_or_else(|| e!("no puzzle lines detected"))?;
    let input_columns = InputColumns::from_str(&column_line)?;
    let mut stacks = Stacks::new(input_columns.len());
    for stack_line in puzzle_iter {
        for (col, c) in input_columns.parse_stack_line(&stack_line) {
            stacks.push(col, c)?;
        }
    }
    Ok(stacks)
}

fn parse_move_instructions(reader: &mut BufReader<File>) -> anyhow::Result<Vec<Move>> {
    let mut moves = Vec::new();
    for line in reader.lines() {
        moves.push(Move::from_str(&line?)?);
    }
    Ok(moves)
}

fn parse_input(filename: &str) -> anyhow::Result<(Stacks, Vec<Move>)> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let stacks = parse_stacks(&mut reader)?;
    let moves = parse_move_instructions(&mut reader)?;
    Ok((stacks, moves))
}

#[derive(Debug)]
struct Move {
    count: usize,
    from_col: usize,
    to_col: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(line: &str) -> anyhow::Result<Self> {
        let caps = MOVE_RE
            .captures(&line)
            .ok_or_else(|| e!("line does not appear to be a move instruction: {}", &line))?;
        let count = caps
            .get(1)
            .ok_or_else(|| e!("no count value in {}", &line))?
            .as_str()
            .parse()?;
        let from_col = caps
            .get(2)
            .ok_or_else(|| e!("no from column value in {}", &line))?
            .as_str()
            .parse()?;
        let to_col = caps
            .get(3)
            .ok_or_else(|| e!("no to column value in {}", &line))?
            .as_str()
            .parse()?;
        Ok(Self {
            count,
            from_col,
            to_col,
        })
    }
}

fn format_tops(tops: &[Option<char>]) -> String {
    String::from_iter(tops.iter().map(|c| c.unwrap_or('?')))
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let (mut stacks, moves) = parse_input(&filename)?;
    dbg!(&stacks.columns);
    dbg!(&moves);
    for instruction in moves {
        stacks.mv(instruction.count, instruction.from_col, instruction.to_col)?;
    }
    dbg!(&stacks.columns);
    println!("{}", format_tops(&stacks.list_tops()));
    Ok(())
}
