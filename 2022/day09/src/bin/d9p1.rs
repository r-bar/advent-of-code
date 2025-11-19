use anyhow::anyhow as e;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord(isize, isize);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Coord {
    fn translate(&mut self, x: isize, y: isize) {
        self.0 += x;
        self.1 += y;
    }

    fn distance(&self, other: &Self) -> usize {
        let xdiff = self.0.abs_diff(other.0);
        let ydiff = self.1.abs_diff(other.1);
        xdiff.max(ydiff)
    }

    fn move_to(&mut self, other: &Self) {
        self.0 = other.0;
        self.1 = other.1;
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "D" => Ok(Self::D),
            "R" => Ok(Self::R),
            "L" => Ok(Self::L),
            _ => Err(e!("invalid direction {s}")),
        }
    }
}

impl Dir {
    fn delta(&self) -> (isize, isize) {
        match self {
            Self::U => (0, 1),
            Self::D => (0, -1),
            Self::L => (-1, 0),
            Self::R => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Rope {
    segments: Vec<Coord>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            segments: vec![Coord(0, 0); length],
        }
    }

    fn run(&mut self, direction: Dir, distance: usize) -> RunIterator<'_> {
        RunIterator {
            rope: self,
            distance,
            direction,
        }
    }

    fn move_head(&mut self, direction: Dir) {
        println!("init {:?} {}", direction, &self);
        let mut old_pos = self.segments[0];
        let (head_delta_x, head_delta_y) = direction.delta();
        self.segments[0].translate(head_delta_x, head_delta_y);
        println!("update head {} {}", &self, &old_pos);

        let mut prev_segment = self.segments[0];
        for segment in self.segments[1..].iter_mut() {
            println!(
                "begin: segment={} prev_segment={} old_pos={}",
                segment, prev_segment, old_pos
            );
            if prev_segment.distance(segment) <= 1 {
                println!("no change");
                break;
            }
            let original_pos = *segment;
            segment.move_to(&old_pos);
            old_pos = original_pos;
            prev_segment = *segment;
            println!(
                "end: segment={} prev_segment={} old_pos={}",
                segment, prev_segment, old_pos
            );
        }
    }

    fn tail_positions(&mut self, commands: &str) -> anyhow::Result<HashSet<Coord>> {
        let mut tail_pos: HashSet<Coord> = HashSet::from([Coord(0, 0)]);
        for line in commands.lines() {
            let cmd: Command = line.parse()?;
            tail_pos.extend(
                self.run(cmd.direction, cmd.distance)
                    .inspect(|rope| println!("{cmd}: {rope}"))
                    .map(|rope| rope.tail().unwrap()),
            );
        }
        Ok(tail_pos)
    }

    fn tail(&self) -> Option<Coord> {
        self.segments.last().copied()
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("H")?;
        for segment in &self.segments {
            write!(f, "{segment}")?;
        }
        f.write_str("T")?;
        Ok(())
    }
}

struct RunIterator<'a> {
    rope: &'a mut Rope,
    distance: usize,
    direction: Dir,
}

impl<'a> Iterator for RunIterator<'a> {
    type Item = Rope;

    fn next(&mut self) -> Option<Self::Item> {
        if self.distance == 0 {
            return None;
        }
        self.rope.move_head(self.direction);
        self.distance -= 1;
        Some(self.rope.clone())
    }
}

struct Command {
    direction: Dir,
    distance: usize,
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts.as_slice() {
            [direction, distance] => Ok(Command {
                direction: direction.parse()?,
                distance: distance.parse()?,
            }),
            _ => Err(e!("invalid command")),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.direction, self.distance)
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let commands = std::fs::read_to_string(&filename)?;
    let mut rope = Rope::new(2);
    println!("{}", rope.tail_positions(&commands)?.len());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example: &str = include_str!("../../example.txt");
        let mut rope = Rope::new(2);
        let tail_pos = rope.tail_positions(example).unwrap();
        let expected = HashSet::from([
            Coord(0, 0),
            Coord(1, 0),
            Coord(1, 2),
            Coord(2, 0),
            Coord(2, 2),
            Coord(2, 4),
            Coord(3, 0),
            Coord(3, 2),
            Coord(3, 3),
            Coord(3, 4),
            Coord(4, 1),
            Coord(4, 2),
            Coord(4, 3),
        ]);
        let mut extra: Vec<_> = tail_pos.difference(&expected).collect();
        let mut missing: Vec<_> = expected.difference(&tail_pos).collect();
        extra.sort();
        missing.sort();
        dbg!(&extra, &missing);
        assert!(extra.is_empty() && missing.is_empty());
    }
}
