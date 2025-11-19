use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Coord = (usize, usize);

enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn walk_from(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, 1),
            Direction::S => (0, -1),
            Direction::E => (-1, 0),
            Direction::W => (1, 0),
        }
    }

    fn walk_to(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        }
    }
}

/// Top left is the root coord
struct Grid {
    trees: Vec<Vec<u8>>,
}

impl Grid {
    fn new() -> Self {
        Self { trees: Vec::new() }
    }

    fn visible(&self, coord: Coord) -> bool {
        self.visible_from(coord, Direction::N)
            || self.visible_from(coord, Direction::S)
            || self.visible_from(coord, Direction::E)
            || self.visible_from(coord, Direction::W)
    }

    fn walk<'a>(&'a self, start: Coord, to_direction: Direction) -> WalkIterator<'a> {
        WalkIterator::new(start, to_direction.walk_to(), self)
    }

    fn visible_from(&self, coord: Coord, from_direction: Direction) -> bool {
        let mut walk = self.walk(coord, from_direction.opposite());
        let start_height = if let Some((_, height)) = walk.next() {
            height
        } else {
            return false;
        };

        walk.all(|(_, height)| start_height > height)
    }

    fn all_visible_from(&self, col_or_row: usize, direction: Direction) -> Vec<(Coord, u8)> {
        let start: Coord = match direction {
            Direction::N => (col_or_row, 0),
            Direction::S => (col_or_row, self.height() - 1),
            Direction::E => (self.width() - 1, col_or_row),
            Direction::W => (0, col_or_row),
        };
        let mut tallest = 0;
        let mut output = Vec::new();
        for (coord, height) in self.walk(start, direction.opposite()) {
            if height > tallest || coord == start {
                tallest = height;
                output.push((coord, height))
            }
            // the heights are read from single digits so we will never see a height taller than 9
            if height >= 9 {
                break;
            }
        }
        output
    }

    fn all_visible(&self) -> HashSet<(Coord, u8)> {
        let n_edge = (0..self.width()).flat_map(|col| self.all_visible_from(col, Direction::N));
        let s_edge = (0..self.width()).flat_map(|col| self.all_visible_from(col, Direction::S));
        let w_edge = (0..self.height()).flat_map(|row| self.all_visible_from(row, Direction::W));
        let e_edge = (0..self.height()).flat_map(|row| self.all_visible_from(row, Direction::E));
        n_edge.chain(s_edge).chain(e_edge).chain(w_edge).collect()
    }

    fn get(&self, coord: Coord) -> Option<u8> {
        self.trees.get(coord.1)?.get(coord.0).copied()
    }

    fn height(&self) -> usize {
        self.trees.len()
    }

    fn width(&self) -> usize {
        self.trees.first().map(|row| row.len()).unwrap_or_default()
    }
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut grid = Grid::new();
        for line in value.lines() {
            let row: Vec<u8> = line
                .trim_end()
                .chars()
                .map(|c| c.to_digit(10).expect("not a digit") as u8)
                .collect();
            grid.trees.push(row);
        }
        Ok(grid)
    }
}

struct WalkIterator<'a> {
    start: Coord,
    last: Option<Coord>,
    delta: (isize, isize),
    grid: &'a Grid,
}

impl<'a> WalkIterator<'a> {
    fn new(start: Coord, delta: (isize, isize), grid: &'a Grid) -> Self {
        WalkIterator {
            grid,
            start,
            delta,
            last: None,
        }
    }
}

impl<'a> Iterator for WalkIterator<'a> {
    type Item = (Coord, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let coord: Coord = match self.last {
            Some(last_coord) => (
                signed_add(last_coord.0, self.delta.0)?,
                signed_add(last_coord.1, self.delta.1)?,
            ),
            None => self.start,
        };
        let height = self.grid.get(coord)?;
        self.last = Some(coord);
        Some((coord, height))
    }
}

fn signed_add(a: usize, b: isize) -> Option<usize> {
    let neg_a: isize = -(a as isize);
    if b < neg_a {
        None
    } else if b < 0 {
        Some(a - b.unsigned_abs())
    } else {
        Some(a + b.unsigned_abs())
    }
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let input = std::fs::read_to_string(&filename)?;
    let grid = Grid::try_from(input.as_str())?;
    println!("{}", grid.all_visible().len());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_signed_add() {
        assert!(
            signed_add(2, 1) == Some(3),
            "positive input, positive output"
        );
        assert!(signed_add(2, 0) == Some(2), "zero input, positive output");
        assert!(
            signed_add(2, -1) == Some(1),
            "negative input, positive output"
        );
        assert!(signed_add(2, -2) == Some(0), "negative input, zero output");
        assert!(
            signed_add(2, -3).is_none(),
            "negative input, negative output"
        );
    }

    #[test]
    fn test_example() {
        let example: &str = include_str!("../../example.txt");
        let grid = Grid::try_from(example).unwrap();
        let all_visible = grid.all_visible();
        let mut sorted_all_visible: Vec<_> = all_visible.clone().into_iter().collect();
        sorted_all_visible.sort();
        println!("{:?}", all_visible);
        let all_visible_count = grid.all_visible().len();
        assert!(grid.visible((1, 0)));
        assert!(grid.visible((1, 1)));
        assert!(grid.visible((2, 1)));
        assert!(grid.visible((3, 2)));

        assert!(!grid.visible((3, 1)));
        assert!(!grid.visible((2, 2)));

        assert!(all_visible_count == 21, "{} != 21", all_visible_count);
    }

    #[test]
    fn test_all_visible_from() {
        let example: &str = include_str!("../../example.txt");
        let grid = Grid::try_from(example).unwrap();
        assert!(dbg!(grid.all_visible_from(1, Direction::E)) == vec![((4, 1), 2), ((2, 1), 5)]);
        assert!(
            dbg!(grid.all_visible_from(4, Direction::W))
                == vec![((0, 4), 3), ((1, 4), 5), ((3, 4), 9)]
        );
        assert!(dbg!(grid.all_visible_from(0, Direction::S)) == vec![((0, 4), 3), ((0, 2), 6)]);
        assert!(dbg!(grid.all_visible_from(4, Direction::N)) == vec![((4, 0), 3), ((4, 3), 9)]);
    }
}
