use std::collections::{BTreeSet, HashMap};
use std::convert::TryFrom;
use std::io::prelude::*;
use std::{env, error, fs, io, iter, ops};

type Point = (isize, isize);
type BoxResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
enum WireDir {
    X(isize),
    Y(isize),
}

impl std::convert::TryFrom<&str> for WireDir {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let maybe_direction: Option<char> = value.chars().nth(0);
        let maybe_distance: Option<isize> = value.get(1..).and_then(|s| s.parse().ok());
        match (maybe_direction, maybe_distance) {
            (Some('U'), Some(distance)) => Ok(WireDir::Y(distance)),
            (Some('D'), Some(distance)) => Ok(WireDir::Y(-distance)),
            (Some('R'), Some(distance)) => Ok(WireDir::X(distance)),
            (Some('L'), Some(distance)) => Ok(WireDir::X(-distance)),
            _ => Err("invalid direction string"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct WireRun(Vec<WireDir>);

impl std::convert::TryFrom<&str> for WireRun {
    type Error = Box<dyn error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut run = Vec::new();
        for segment in value.split(',') {
            run.push(WireDir::try_from(segment)?)
        }
        Ok(WireRun(run))
    }
}

impl WireRun {
    fn path_points(&self) -> Vec<Point> {
        let mut current: Point = (0, 0);
        let mut points = vec![current];
        for segment in self.0.iter() {
            let segment_points: Box<dyn Iterator<Item = Point>> = match *segment {
                WireDir::Y(0) => continue,
                WireDir::X(0) => continue,
                WireDir::Y(distance) if distance > 0 => {
                    let y_coords = (current.1 + 1)..=(current.1 + distance);
                    Box::new(iter::repeat(current.0).zip(y_coords))
                }
                WireDir::Y(distance) if distance < 0 => {
                    let y_coords = ((current.1 + distance)..current.1).rev();
                    Box::new(iter::repeat(current.0).zip(y_coords))
                }
                WireDir::X(distance) if distance > 0 => {
                    let x_coords = (current.0 + 1)..=(current.0 + distance);
                    Box::new(x_coords.zip(iter::repeat(current.1)))
                }
                WireDir::X(distance) if distance < 0 => {
                    let x_coords = ((current.0 + distance)..current.0).rev();
                    Box::new(x_coords.zip(iter::repeat(current.1)))
                }
                // Compiler will not detect exhaustive match of numeric ranges
                // https://github.com/rust-lang/rust/issues/12483
                // The above paterns should be exhaustive
                WireDir::X(_) => unreachable!(),
                WireDir::Y(_) => unreachable!(),
            };
            let segment_points: Vec<_> = segment_points.collect();
            points.extend(segment_points);
            if let Some(point) = points.last() {
                current = *point
            }
        }
        points
    }

    fn intersections(&self, other: &WireRun) -> Vec<Point> {
        let self_pts = self.path_points();
        let mut other_pts = BTreeSet::new();
        other_pts.extend(other.path_points());
        self_pts
            .iter()
            .filter(move |pt| other_pts.contains(pt))
            .map(|pt| *pt)
            .collect()
    }

    fn closest_intersection(
        &self,
        anchor: Point,
        exclude_anchor: bool,
        other: &WireRun,
    ) -> Option<Point> {
        let mut intersections = self.intersections(other);
        if exclude_anchor {
            intersections = intersections
                .iter()
                .filter(|pt| **pt != anchor)
                .map(|pt| *pt)
                .collect();
        }
        intersections.sort_by_key(|pt| manhattan_distance((0, 0), *pt));
        intersections.first().map(|pt| *pt)
    }

    fn shortest_intersection(
        &self,
        exclude_origin: bool,
        other: &WireRun,
    ) -> Option<(usize, Point)> {
        let mut distances_a: HashMap<Point, usize> = HashMap::new();
        distances_a.reserve(self.0.len());
        for (dist, pt) in self.path_points().iter().enumerate() {
            match distances_a.get(pt) {
                Some(visited_dist) => {
                    if dist < *visited_dist {
                        distances_a.insert(*pt, dist);
                    }
                }
                None => {
                    distances_a.insert(*pt, dist);
                }
            }
        }
        let mut intersections: Vec<(usize, Point)> = Vec::new();
        for (dist_b, pt) in other.path_points().iter().enumerate() {
            if exclude_origin && *pt == (0, 0) {
                continue;
            }
            match distances_a.get(pt) {
                Some(dist_a) => intersections.push((dist_a + dist_b, *pt)),
                None => continue,
            }
        }
        intersections.sort();
        intersections.first().map(|val| *val)
    }
}

fn wire_runs_from_file(path: &str) -> BoxResult<Vec<WireRun>> {
    let f = fs::File::open(path)?;
    let reader = io::BufReader::new(f);
    let mut runs = Vec::new();
    for line in reader.lines() {
        runs.push(WireRun::try_from(&line? as &str)?);
    }
    Ok(runs)
}

fn manhattan_distance(a: Point, b: Point) -> isize {
    let xdistance = (a.0 - b.0).abs();
    let ydistance = (a.1 - b.1).abs();
    xdistance + ydistance
}

fn main() {
    let input_path = env::args().nth(1).expect("input file required");
    let runs = wire_runs_from_file(&input_path).expect("invalid input file");
    let (wire1, wire2) = match runs.as_slice() {
        [wire1, wire2] => (wire1, wire2),
        _ => {
            eprintln!("two runs are required in the input file");
            std::process::exit(1);
        }
    };
    println!("part 1");
    match wire1.closest_intersection((0, 0), true, &wire2) {
        Some(point) => {
            println!("closest point: {:?}", point);
            println!("distance: {}", manhattan_distance((0, 0), point))
        }
        None => {
            eprintln!("no intersection found");
            std::process::exit(1);
        }
    }

    println!("\npart 2");
    match wire1.shortest_intersection(true, wire2) {
        Some((distance, point)) => {
            println!("shortest_intersection: {:?}", point);
            println!("distance: {}", distance);
        }
        None => {
            eprintln!("no intersection found");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path1() {
        let input = "L2,U3";
        let wire = WireRun::try_from(input).unwrap();
        assert_eq!(wire.0, vec![WireDir::X(-2), WireDir::Y(3)]);
        assert_eq!(
            wire.path_points(),
            vec![(0, 0), (-1, 0), (-2, 0), (-2, 1), (-2, 2), (-2, 3)],
        );
    }

    #[test]
    fn test_simple_path2() {
        let input = "R2,D3";
        let wire = WireRun::try_from(input).unwrap();
        assert_eq!(wire.0, vec![WireDir::X(2), WireDir::Y(-3)]);
        assert_eq!(
            wire.path_points(),
            vec![(0, 0), (1, 0), (2, 0), (2, -1), (2, -2), (2, -3)],
        );
    }

    #[test]
    fn test_path_points() {
        let wire1 = WireRun::try_from("R8,U5,L5,D3").unwrap();
        let wire2 = WireRun::try_from("U7,R6,D4,L4").unwrap();
        let mut intersections = BTreeSet::new();
        intersections.extend(wire1.intersections(&wire2));
        println!("{:?}", intersections);
    }

    #[test]
    fn test_closest_intersection() {
        let wire1_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let wire2_input = "U62,R66,U55,R34,D71,R55,D58,R83";
        let wire1 = WireRun::try_from(wire1_input).unwrap();
        let wire2 = WireRun::try_from(wire2_input).unwrap();
        let intersections = wire1.intersections(&wire2);
        println!("{:?}", intersections);
        let intersection = wire1.closest_intersection((0, 0), true, &wire2).unwrap();
        assert_eq!(manhattan_distance((0, 0), intersection), 159)
    }

    #[test]
    fn test_short_intersection_1() {
        let wire1 = WireRun::try_from("R8,U5,L5,D3").unwrap();
        let wire2 = WireRun::try_from("U7,R6,D4,L4").unwrap();
        let (distance, _point) = wire1.shortest_intersection(true, &wire2).unwrap();
        assert_eq!(distance, 30);
    }

    #[test]
    fn test_short_intersection_2() {
        let wire1_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let wire2_input = "U62,R66,U55,R34,D71,R55,D58,R83";
        let wire1 = WireRun::try_from(wire1_input).unwrap();
        let wire2 = WireRun::try_from(wire2_input).unwrap();
        let (distance, _point) = wire1.shortest_intersection(true, &wire2).unwrap();
        assert_eq!(distance, 610);
    }

    #[test]
    fn test_short_intersection_3() {
        let wire1_input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let wire2_input = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let wire1 = WireRun::try_from(wire1_input).unwrap();
        let wire2 = WireRun::try_from(wire2_input).unwrap();
        let (distance, _point) = wire1.shortest_intersection(true, &wire2).unwrap();
        assert_eq!(distance, 410);
    }
}
