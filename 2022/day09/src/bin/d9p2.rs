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

    fn follow(&mut self, other: &Self) {
        if self.distance(other) < 2 {
            return;
        }
        self.0 = max_diff(self.0, other.0, 1);
        self.1 = max_diff(self.1, other.1, 1);
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn set(&mut self, positions: &[Coord]) {
        for (i, coord) in positions.iter().enumerate() {
            self.segments[i] = *coord;
        }
    }

    fn move_head(&mut self, direction: Dir) {
        //println!("init {:?} {}", direction, &self);
        let (head_delta_x, head_delta_y) = direction.delta();
        self.segments[0].translate(head_delta_x, head_delta_y);
        //println!("update head {} {}", &self, &old_pos);

        let mut prev_segment = self.segments[0];
        for (i, segment) in self.segments.iter_mut().enumerate().skip(1) {
            segment.follow(&prev_segment);
            prev_segment = *segment;
        }
    }

    fn tail_positions(&mut self, commands: &[Command]) -> anyhow::Result<HashSet<Coord>> {
        let mut tail_pos: HashSet<Coord> = HashSet::from([self.tail().unwrap()]);
        for cmd in commands {
            tail_pos.extend(
                self.run(cmd.direction, cmd.distance)
                    .map(|rope| rope.tail().unwrap()),
            );
        }
        Ok(tail_pos)
    }

    fn tail(&self) -> Option<Coord> {
        self.segments.last().copied()
    }
}

//impl From<&[Coord]> for Rope {
//    fn from(arr: &[Coord]) -> Self {
//        Rope {
//            segments: arr.into(),
//        }
//    }
//}

impl FromIterator<Coord> for Rope {
    fn from_iter<T: IntoIterator<Item = Coord>>(iter: T) -> Self {
        Rope {
            segments: iter.into_iter().collect(),
        }
    }
}

impl<T> From<T> for Rope
where
    T: IntoIterator<Item = Coord>,
{
    fn from(i: T) -> Self {
        Rope::from_iter(i)
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

impl Command {
    fn parse_all(s: &str) -> anyhow::Result<Vec<Self>> {
        s.lines().map(|line| line.parse()).collect()
    }
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

fn max_diff(a: isize, b: isize, max: usize) -> isize {
    if a > b {
        a - (a - b).abs().min(max as isize)
    } else if a < b {
        a + (a - b).abs().min(max as isize)
    } else {
        a
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = std::fs::read_to_string(&filename)?;
    let commands = Command::parse_all(&input)?;
    let mut rope = Rope::new(10);
    println!("{}", rope.tail_positions(&commands)?.len());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_part2() -> anyhow::Result<()> {
        let example: &str = include_str!("../../example.txt");
        let commands = Command::parse_all(example)?;
        let mut rope = Rope::new(10);
        let tail_pos = rope.tail_positions(&commands)?;
        let expected = HashSet::from([Coord(0, 0)]);
        let mut extra: Vec<_> = tail_pos.difference(&expected).collect();
        let mut missing: Vec<_> = expected.difference(&tail_pos).collect();
        extra.sort();
        missing.sort();
        dbg!(&extra, &missing);
        assert!(extra.is_empty() && missing.is_empty());
        Ok(())
    }

    #[test]
    fn test_example2_part2_tail_positions() -> anyhow::Result<()> {
        let example: &str = include_str!("../../example2.txt");
        let commands = Command::parse_all(example)?;
        let mut rope = Rope::new(10);
        let tail_pos = rope.tail_positions(&commands)?;
        assert!(dbg!(tail_pos.len()) == 36);
        Ok(())
    }

    #[test]
    fn test_example2_part2() -> anyhow::Result<()> {
        let example: &str = include_str!("../../example2.txt");
        let commands = Command::parse_all(example)?;
        let mut rope = Rope::new(10);
        let expected_positions = [
            Rope::from([
                Coord(5, 0),
                Coord(4, 0),
                Coord(3, 0),
                Coord(2, 0),
                Coord(1, 0),
                Coord(0, 0),
                Coord(0, 0),
                Coord(0, 0),
                Coord(0, 0),
                Coord(0, 0),
            ]),
            Rope::from([
                Coord(5, 8),
                Coord(5, 7),
                Coord(5, 6),
                Coord(5, 5),
                Coord(5, 4),
                Coord(4, 4),
                Coord(3, 3),
                Coord(2, 2),
                Coord(1, 1),
                Coord(0, 0),
            ]),
        ];
        let mut tail_pos = HashSet::new();
        for (command, expected) in commands.iter().zip(expected_positions) {
            println!("{command}");
            tail_pos.extend(
                rope.run(command.direction, command.distance)
                    .map(|rope| rope.tail().unwrap()),
            );
            assert!(rope.segments.len() == 10);
            assert!(expected.segments.len() == 10);
            assert!(dbg!(&rope) == dbg!(&expected));
        }
        Ok(())
    }

    #[test]
    fn test_max_diff() {
        assert!(max_diff(1, 2, 1) == 2);
        assert!(max_diff(1, 3, 1) == 2);
        assert!(max_diff(2, 1, 1) == 1);
        assert!(max_diff(2, -1, 1) == 1);
        assert!(max_diff(-2, -4, 1) == -3);
        assert!(max_diff(-2, 0, 1) == -1);
        assert!(max_diff(-2, -1, 1) == -1);
        assert!(max_diff(-2, -2, 1) == -2);
    }
}
