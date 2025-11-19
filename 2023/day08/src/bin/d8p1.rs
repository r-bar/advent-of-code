use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::HashSet;
use num::integer::lcm;

lazy_static! {
    static ref NODE_REGEX: regex::Regex =
        regex::Regex::new(r"^(?<name>\w+) = \((?<left>\w+), (?<right>\w+)\)$").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matches = NODE_REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!("Invalid node line: {s}"))?;
        let name = matches.name("name").unwrap().as_str().to_string();
        let left = matches.name("left").unwrap().as_str().to_string();
        let right = matches.name("right").unwrap().as_str().to_string();
        Ok(Node { name, left, right })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Map {
    nodes: HashMap<String, Node>,
    first_node: String,
    directions: Vec<Direction>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let mut lines = s.lines();
        let direction_line = lines.next().ok_or(anyhow::anyhow!("No direction line."))?;
        let mut directions = Vec::new();
        for c in direction_line.chars() {
            match c {
                'L' => directions.push(Direction::Left),
                'R' => directions.push(Direction::Right),
                _ => continue,
            }
        }
        let _ = lines
            .next()
            .ok_or(anyhow::anyhow!("No direction speerator line."))?;
        let mut first_node = None;
        for line in lines {
            let node: Node = line.parse()?;
            if first_node.is_none() {
                first_node = Some(node.name.clone());
            }
            nodes.insert(node.name.clone(), node);
        }
        let first_node = first_node.ok_or(anyhow::anyhow!("No first nodes found."))?;
        Ok(Map {
            nodes,
            directions,
            first_node,
        })
    }
}

fn part1(map: &Map) -> anyhow::Result<usize> {
    let mut steps = 0;
    let start_node = String::from("AAA");
    let mut node_ptr = start_node.as_str();
    let mut dir_ptr = map.directions.iter().cycle();

    while node_ptr != "ZZZ" {
        let node = map
            .nodes
            .get(node_ptr)
            .ok_or(anyhow::anyhow!("Invalid node."))?;
        let direction = dir_ptr.next().unwrap();
        match direction {
            Direction::Left => node_ptr = &node.left,
            Direction::Right => node_ptr = &node.right,
        }
        steps += 1;
    }

    Ok(steps)
}

fn part2(map: &Map) -> anyhow::Result<usize> {
    let mut ptrs: Vec<_> = map
        .nodes
        .keys()
        .filter(|&k| k.ends_with('A'))
        .map(|s| s.as_str())
        .collect();
    //let mut circuit_breaker = HashSet::new();
    let mut first_z: Vec<usize> = std::iter::repeat(0).take(ptrs.len()).collect();
    for step in 0..1000000000 {
        //println!("Step: {} Paths: {:?}", step, ptrs);
        for (i, ptr) in ptrs.iter().enumerate() {
            if ptr.ends_with('Z') && first_z[i] == 0 {
                first_z[i] = step;
            }
        }
        if ptrs.iter().all(|&p| p.ends_with('Z')) || first_z.iter().all(|&s| s > 0) {
            return first_z.into_iter().reduce(lcm).ok_or(anyhow::anyhow!("Cannot calculate LCM."));
        }
        let direction = map.directions[step % map.directions.len()];
        for node_ptr in ptrs.iter_mut() {
            let node = map
                .nodes
                .get(*node_ptr)
                .ok_or(anyhow::anyhow!("Invalid node."))?;
            let new_ptr = match direction {
                Direction::Left => node.left.as_str(),
                Direction::Right => node.right.as_str(),
            };
            *node_ptr = new_ptr;
        }
    }

    Err(anyhow::anyhow!("No path found."))
}

fn main() -> anyhow::Result<()> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input.txt".to_string());
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let map: Map = buffer.parse()?;
    let result = part1(&map)?;
    println!("Part 1: {}", result);
    let result = part2(&map)?;
    println!("Part 2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    const EXAMPLE1: &str = include_str!("../../example1.txt");
    const EXAMPLE2: &str = include_str!("../../example2.txt");
    const EXAMPLE3: &str = include_str!("../../example3.txt");

    #[test]
    fn test_parse_example() -> anyhow::Result<()> {
        let map: Map = EXAMPLE1.parse()?;
        assert_eq!(&map.first_node, "AAA");
        assert_eq!(map.directions, vec![Direction::Right, Direction::Left]);
        assert_eq!(map.nodes.len(), 7);
        Ok(())
    }

    #[test]
    fn test_part1_example() {
        let map: Map = EXAMPLE1.parse().unwrap();
        assert_eq!(part1(&map).unwrap(), 2);
    }

    #[test]
    fn test_part1_example2() {
        let map: Map = EXAMPLE2.parse().unwrap();
        assert_eq!(part1(&map).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        let map: Map = EXAMPLE3.parse().unwrap();
        assert_eq!(part2(&map).unwrap(), 6);
    }

    #[test]
    fn test_regex() {
        let node: Node = "AAA = (BBB, CCC)".parse().unwrap();
        assert_eq!(node.name, "AAA");
        assert_eq!(node.left, "BBB");
        assert_eq!(node.right, "CCC");
    }
}
