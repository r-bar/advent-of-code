use anyhow::Context;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
//use std::fs::File;
use std::io::prelude::*;
//use std::io::BufReader;
use std::io::BufWriter;
use std::str::FromStr;
//use std::thread::sleep;
//use std::time::Duration;

type Coord = (usize, usize);

struct Map {
    elevations: Vec<Vec<u8>>,
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
enum BFSResult {
    Continue(Vec<Vec<Coord>>),
    Complete(Vec<Coord>),
}

impl Map {
    fn get(&self, coord: Coord) -> Option<u8> {
        self.elevations.get(coord.1)?.get(coord.0).copied()
    }

    fn get_char(&self, coord: Coord) -> Option<char> {
        if coord == self.start {
            return Some('S');
        } else if coord == self.end {
            return Some('E');
        }
        self.get(coord).map(|h| (h + 97) as char)
    }

    /// Returns list of candidates for each node sorted by .heuristic()
    fn candidates(&self, node: Coord) -> Vec<Coord> {
        let height = match self.get(node) {
            Some(h) => h,
            None => return Vec::new(),
        };
        let candidates: Vec<Coord> = [
            (node.0.checked_sub(1), Some(node.1)),
            (Some(node.0), node.1.checked_sub(1)),
            (Some(node.0), Some(node.1 + 1)),
            (Some(node.0 + 1), Some(node.1)),
        ]
        .into_iter()
        .flat_map(|coord| match coord {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        })
        .collect();
        //candidates.sort_by_key(|node| self.heuristic(*node));
        candidates
    }

    fn _bfs(
        &self,
        visited: &mut HashSet<Coord>,
        path: Vec<Coord>,
        check: fn(&Map, Coord, Coord) -> bool,
        complete: fn(&Map, Coord) -> bool,
    ) -> BFSResult {
        let last_node = *path
            .last()
            .expect("bfs paths must have at least one member");
        // this node could have already been visited from another path
        if visited.contains(&last_node) {
            return BFSResult::Continue(Vec::new());
        }
        if complete(self, last_node) {
            return BFSResult::Complete(path);
        }
        visited.insert(last_node);
        let candidates = self
            .candidates(last_node)
            .into_iter()
            .filter(|next_node| {
                check(&self, last_node, *next_node)
            })
            .filter_map(|n| {
                if visited.contains(&n) {
                    None
                } else {
                    let mut candidate_path = path.clone();
                    candidate_path.push(n);
                    Some(candidate_path)
                }
            })
            .collect();
        BFSResult::Continue(candidates)
    }

    fn climb(&self) -> Option<Vec<Coord>> {
        let mut visited = HashSet::new();
        let mut candidates = VecDeque::from([Vec::from([self.start])]);
        let complete = |map: &Self, last: Coord| last == map.end;
        let check = |map: &Self, from: Coord, to: Coord| map.can_climb(from, to);
        while let Some(path) = candidates.pop_front() {
            match self._bfs(&mut visited, path, check, complete) {
                BFSResult::Complete(solution) => return Some(solution),
                BFSResult::Continue(new_candidates) => {
                    candidates.extend(new_candidates);
                }
            }
            //dbg!(candidates.len(), visited.len());
        }
        None
    }

    fn descend(&self) -> Option<Vec<Coord>> {
        let mut visited = HashSet::new();
        let mut candidates = VecDeque::from([Vec::from([self.start])]);
        while let Some(path) = candidates.pop_front() {
            match self._bfs(&mut visited, path) {
                BFSResult::Complete(solution) => return Some(solution),
                BFSResult::Continue(new_candidates) => {
                    candidates.extend(new_candidates);
                }
            }
            //dbg!(candidates.len(), visited.len());
        }
        None

    }

    fn height(&self) -> usize {
        self.elevations.len()
    }

    fn width(&self) -> usize {
        self.elevations
            .first()
            .map(|row| row.len())
            .unwrap_or_default()
    }

    fn can_climb(&self, from: Coord, to: Coord) -> bool {
        match (self.get(from), self.get(to)) {
            (Some(from_height), Some(to_height)) => to_height.saturating_sub(from_height) <= 1,
            _ => false,
        }
    }

    fn can_descend(&self, from: Coord, to: Coord) -> bool {
        match (self.get(from), self.get(to)) {
            (Some(from_height), Some(to_height)) => from_height.saturating_sub(to_height) <= 1,
            _ => false,
        }
    }
}

// Calculate minimum distance between nodes given we can only travel vertically and horicontally.
// Ignores height.
#[allow(dead_code)]
fn distance(a: Coord, b: Coord) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn height(c: char) -> u8 {
    match c {
        'a'..='z' => (c as u8) - 97,
        'E' => 26,
        _ => 0,
    }
}

fn print_path2(map: &Map, path: &[Coord]) -> anyhow::Result<()> {
    use termion::color;

    let traversed: HashSet<&Coord> = HashSet::from_iter(path);
    let mut buffer = BufWriter::new(std::io::stdout());
    let path_color = color::Fg(color::Red);
    let map_color = color::Fg(color::Reset);
    for y in 0..map.height() {
        for x in 0..map.width() {
            let chr = map.get_char((x, y)).unwrap_or('?');
            if traversed.contains(&(x, y)) {
                write!(&mut buffer, "{}", &path_color)?;
            } else {
                write!(&mut buffer, "{}", &map_color)?;
            };
            write!(&mut buffer, "{chr}")?;
        }
        writeln!(&mut buffer)?;
    }
    write!(&mut buffer, "{}", color::Fg(color::Reset))?;
    Ok(())
}

impl FromStr for Map {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elevations = Vec::new();
        let mut start = None;
        let mut end = None;
        for (i, line) in s.lines().enumerate() {
            elevations.push(line.chars().map(height).collect());
            if let Some(startx) = line.find('S') {
                start = Some((startx, i));
            }
            if let Some(endx) = line.find('E') {
                end = Some((endx, i));
            }
        }
        Ok(Self {
            elevations,
            start: start.context("start marker not found")?,
            end: end.context("end marker not found")?,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let map_str = std::fs::read_to_string(filename)?;
    let map = Map::from_str(&map_str).expect("failed to read map");
    let solution = map.bfs().expect("failed to find route");
    print_path2(&map, &solution)?;
    println!("{}", solution.len() - 1);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_test() {
        let example = include_str!("../../example.txt");
        let map = Map::from_str(example).expect("failed to read map");
        let solution = map.bfs().expect("failed to find route");
        dbg!(&solution);
        print_path2(&map, &solution).unwrap();
        assert_eq!(solution.first().copied(), Some((0, 0)));
        assert_eq!(solution.last().copied(), Some((5, 2)));
        assert_eq!(solution.len() - 1, 31);
    }

    #[test]
    fn input_test() {
        let example = include_str!("../../input.txt");
        let map = Map::from_str(example).expect("failed to read map");
        let solution = map.bfs().expect("failed to find route");
        dbg!(&solution);
        print_path2(&map, &solution).unwrap();
        assert_eq!(solution.len() - 1, 472);
    }
}
