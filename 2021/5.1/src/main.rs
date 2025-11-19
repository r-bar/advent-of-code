use anyhow::anyhow as err;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::{env, fs};
use std::collections::HashMap;

enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point(usize, usize);

#[derive(Debug)]
struct Segment(Point, Point);

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_ascii_whitespace().collect();
        let mut left_coords = parts[0]
            .split(',')
            .filter_map(|s| usize::from_str_radix(&s, 10).ok());
        let left = Point(
            left_coords.next().ok_or(err!("Bad coord"))?,
            left_coords.next().ok_or(err!("Bad coord"))?,
        );
        let mut right_coords = parts[2]
            .split(',')
            .filter_map(|s| usize::from_str_radix(&s, 10).ok());
        let right = Point(
            right_coords.next().ok_or(err!("Bad coord"))?,
            right_coords.next().ok_or(err!("Bad coord"))?,
        );
        Ok(Segment(left, right))
    }
}

impl Segment {

    fn _slope(a: isize, b: isize) -> isize {
        let diff = a - b;
        match 0.cmp(&diff) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
        }
    }

    /// Returns the steps for the given segment. Works only for horizontal, vertical, and 45
    /// degree segments
    fn slope(&self) -> (isize, isize) {
        let x_step = Self::_slope(self.0.0 as isize, self.1.0 as isize);
        let y_step = Self::_slope(self.0.1 as isize, self.1.1 as isize);
        (x_step, y_step)
    }

    fn coords(&self) -> anyhow::Result<Vec<Point>> {
        let mut output = Vec::new();
        let (mut x, mut y): (isize, isize) = (self.0.0 as isize, self.0.1 as isize);
        let (x_step, y_step) = self.slope();
        let mut i = 0;
        while Point(x.try_into()?, y.try_into()?) != self.1 {
            output.push(Point(x.try_into()?, y.try_into()?));
            x += x_step;
            y += y_step;
            i += 1;
            if i > 10000 {
                return Err(err!("wut"))
            }
        }
        output.push(Point(x.try_into()?, y.try_into()?));
        Ok(output)
    }
}

struct Map {
    vents: Vec<Segment>,
}

impl Map {
    fn from_file(path: &str) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(&file);
        let mut vents = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let vent = Segment::from_str(line.trim())?;
            vents.push(vent)
        }
        Ok(Map { vents })
    }

    /// Returns a set of points representing the minimum and maximum corners of the grid
    fn bounds(&self) -> (Point, Point) {
        let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
        let (mut max_x, mut max_y) = (0, 0);
        for segment in &self.vents {
            let mut xs = [segment.0.0, segment.1.0];
            let mut ys = [segment.0.1, segment.1.1];
            xs.sort();
            ys.sort();
            if xs[0] < min_x {
                min_x = xs[0];
            }
            if xs[1] > max_x {
                max_x = xs[1];
            }
            if ys[0] < min_y {
                min_y = ys[0];
            }
            if ys[1] > max_y {
                max_y = ys[1];
            }
        }
        (Point(min_x, min_y), Point(max_x, max_y))
    }

    /// Returns the set of all points where at least 1 segment overlaps
    fn overlaps(&self) -> HashMap<Point, usize> {
        let mut counts = HashMap::new();
        let points = self.vents.iter()
            // filter out the diagonal segments
            .filter(|seg| {let slope = seg.slope(); slope.0 == 0 || slope.1 == 0})
            // collect all the points, doesnt matter what segment they belong to
            .filter_map(|v| v.coords().ok()).flatten();
        for point in points {
            *counts.entry(point).or_insert(0) += 1;
        }
        counts.retain(|_k, v| *v > 1);
        counts
    }

}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("First argument must contain path to input file");
    let map = Map::from_file(&path).expect("could not read map data");
    println!("vents: {}", map.vents.len());
    println!("overlaps: {}", map.overlaps().len());
}
