use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::str::FromStr;

type Point = (usize, usize);
type Index = BTreeMap<Point, usize>;

#[derive(Clone)]
pub struct Map {
    locations: Vec<Point>,
    index: Index,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locations = Vec::new();
        let mut index = BTreeMap::new();
        let mut width = 0;
        let mut height = 0;
        for (lineno, line) in s.trim().lines().enumerate() {
            for (colno, char) in line.chars().enumerate() {
                if char == '#' {
                    index.insert((colno, lineno), locations.len());
                    locations.push((colno, lineno))
                }
            }
            if line.len() > width {
                width = line.len();
            }
            height += 1;
        }
        Ok(Map {
            height,
            width,
            locations,
            index,
        })
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut locs = self.locations.iter();
        let mut next_loc = locs.next();
        for y in 0..self.height {
            for x in 0..self.width {
                if next_loc == Some(&(x, y)) {
                    next_loc = locs.next();
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn get_by_coord(&self, x: usize, y: usize) -> Option<usize> {
        self.index.get(&(x, y)).copied()
    }

    pub fn get_by_id(&self, id: usize) -> Option<Point> {
        self.locations.get(id).copied()
    }

    pub fn set(&mut self, loc_id: usize, x: usize, y: usize) -> anyhow::Result<()> {
        let curr_loc = self
            .locations
            .get_mut(loc_id)
            .ok_or(anyhow::anyhow!("invalid location id"))?;
        self.index.remove(curr_loc);
        self.index.insert((x, y), loc_id);
        *curr_loc = (x, y);
        Ok(())
    }

    pub fn col(&self, x: usize) -> Vec<Option<usize>> {
        let mut output = Vec::with_capacity(self.height);
        for y in 0..self.height {
            output.push(self.get_by_coord(x, y));
        }
        output
    }

    pub fn row(&self, y: usize) -> Vec<Option<usize>> {
        let mut output = Vec::with_capacity(self.width);
        for x in 0..self.width {
            output.push(self.get_by_coord(x, y));
        }
        output
    }

    pub fn iter_locations(&self) -> impl Iterator<Item = &Point> {
        self.locations.iter()
    }

    ///
    /// n is the number of times to expand.
    pub fn expand(&mut self, n: usize) {
        let mut occupied_cols = BTreeSet::new();
        let mut occupied_rows = BTreeSet::new();
        for location in self.locations.iter() {
            occupied_cols.insert(location.0);
            occupied_rows.insert(location.1);
        }
        let occupied_cols: Vec<_> = occupied_cols.into_iter().collect();
        let occupied_rows: Vec<_> = occupied_rows.into_iter().collect();
        let mut index = BTreeMap::new();
        //let mut locations = self.locations.clone();
        for (i, loc) in self.locations.iter_mut().enumerate() {
            let (curr_x, curr_y) = *loc;
            let occupied_col_before = occupied_cols.partition_point(|col| col < &curr_x);
            let occupied_row_before = occupied_rows.partition_point(|row| row < &curr_y);
            let empty_col_before = curr_x - occupied_col_before;
            let empty_row_before = curr_y - occupied_row_before;
            let new_x = empty_col_before * n + occupied_col_before;
            let new_y = empty_row_before * n + occupied_row_before;
            *loc = (new_x, new_y);
            index.insert((new_x, new_y), i);
        }
        self.height = (self.height - occupied_rows.len()) * n + occupied_rows.len();
        self.width = (self.width - occupied_cols.len()) * n + occupied_cols.len();
        self.index = index;
    }
}

pub fn distance(a: Point, b: Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_expansion() {
        let example: Map = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"
        .parse()
        .unwrap();
        dbg!(&example.locations);
        let expected: Map = "\
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"
        .parse()
        .unwrap();
        let mut expanded = example.clone();
        expanded.expand(2);
        assert_eq!(expanded.width, expected.width);
        assert_eq!(expanded.height, expected.height);
        assert_eq!(expanded.locations, expected.locations);
    }
}
