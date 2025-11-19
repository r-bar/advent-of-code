#[allow(dead_code)]
use std::borrow::Borrow;
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::prelude::*;
use std::rc::Rc;

type NodeRef = RefCell<Node>;

#[derive(Debug)]
struct Node {
    name: String,
    outbound: HashSet<String>,
    inbound: HashSet<String>,
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            name: name.into(),
            outbound: HashSet::new(),
            inbound: HashSet::new(),
        }
    }

    fn new_rc(name: &str) -> NodeRef {
        RefCell::new(Self::new(name))
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, NodeRef>,
}

#[derive(Debug)]
enum Direction {
    Inbound,
    Outbound,
    Any,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Inbound, Direction::Inbound) => true,
            (Direction::Outbound, Direction::Outbound) => true,
            (Direction::Any, _) => true,
            (_, Direction::Any) => true,
            _ => false,
        }
    }
}

struct BfsGraphIterator<'a> {
    graph: &'a Graph,
    to_visit: Vec<String>,
    visited: HashSet<String>,
    direction: Direction,
}

impl<'a> Iterator for BfsGraphIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let node_name = match self.to_visit.pop() {
            Some(name) => name.clone(),
            None => return None,
        };
        self.visited.insert(node_name.clone());
        let node = match self.graph.nodes.get(&node_name) {
            Some(n) => n.borrow(),
            None => return None,
        };
        if self.direction == Direction::Outbound {
            for outbound in node.outbound.iter() {
                if !self.visited.contains(outbound) {
                    self.to_visit.push(outbound.into());
                }
            }
        }
        if self.direction == Direction::Inbound {
            for inbound in node.inbound.iter() {
                if !self.visited.contains(inbound) {
                    self.to_visit.push(inbound.into());
                }
            }
        }
        Some(node_name.into())
    }
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        {
            let mut from_node = self
                .nodes
                .entry(from.into())
                .or_insert(Node::new_rc(from.into()))
                .borrow_mut();
            from_node.outbound.insert(to.into());
        }
        {
            let mut to_node = self
                .nodes
                .entry(to.into())
                .or_insert(Node::new_rc(to.into()))
                .borrow_mut();
            to_node.inbound.insert(from.into());
        }
    }

    fn iter_nodes(&self) -> impl Iterator<Item = std::cell::Ref<'_, Node>> {
        self.nodes.values().map(|cell| cell.borrow())
    }

    fn related_nodes(&self, name: &str) -> (HashSet<String>, HashSet<String>) {
        self.nodes
            .get(name)
            .map(|node| {
                (
                    node.borrow().inbound.clone(),
                    node.borrow().outbound.clone(),
                )
            })
            .unwrap_or_else(|| (HashSet::new(), HashSet::new()))
    }

    fn remove(&mut self, name: &str) {
        let (inbound, outbound) = self.related_nodes(name);
        let inbound_nodes = inbound
            .iter()
            .filter_map(|in_name| self.nodes.get(in_name))
            .map(|cell| cell.borrow_mut());
        for mut inbound_node in inbound_nodes {
            inbound_node.outbound.remove(name);
        }
        let outbound_nodes = outbound
            .iter()
            .filter_map(|out_name| self.nodes.get(out_name))
            .map(|cell| cell.borrow_mut());
        for mut outbound_node in outbound_nodes {
            outbound_node.inbound.remove(name);
        }
        self.nodes.remove(name);
    }

    /// Recursively walk down the graph to find to outbound depth of the given node name
    fn outbound_count_old(&self, name: &str) -> usize {
        let node = match self.nodes.get(name) {
            Some(n) => n.borrow(),
            None => return 0,
        };
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(name.into());
        let mut to_visit: HashSet<String> = HashSet::new();
        to_visit.extend(node.outbound.clone());
        let mut depth: usize = 0;
        while to_visit.len() > 0 {
            let mut visit_next_round: HashSet<String> = HashSet::new();
            for visit_name in to_visit.iter() {
                visited.insert(visit_name.clone());
                let visit_node = match self.nodes.get(visit_name) {
                    Some(node) => node.borrow(),
                    None => continue,
                };
                for outbound_name in visit_node.outbound.iter() {
                    if !visited.contains(outbound_name) {
                        visit_next_round.insert(outbound_name.into());
                    }
                }
            }
            to_visit = visit_next_round;
            depth += 1;
        }
        depth
    }

    fn outbound_count(&self, name: &str) -> usize {
        self.bfs_iter(name, false, Direction::Outbound).count()
    }

    fn bfs_iter<'a>(
        &'a self,
        start: &str,
        include_start: bool,
        direction: Direction,
    ) -> impl Iterator<Item = String> + 'a {
        let to_visit: Vec<String> = if include_start {
            vec![start.into()]
        } else {
            self.nodes
                .get(start)
                .map(|n| n.borrow().outbound.iter().map(|i| i.clone()).collect())
                .unwrap_or(vec![])
        };
        BfsGraphIterator {
            graph: self,
            to_visit: to_visit,
            visited: HashSet::new(),
            direction: direction,
        }
    }

    fn count_edges(&self) -> usize {
        let mut orbits = 0;
        for node in self.nodes.values().map(|n| n.borrow()) {
            orbits += node.outbound.len();
        }
        orbits
    }

    fn traversal(
        &self,
        start: &str,
        end: &str,
        direction: Direction,
        max_depth: Option<usize>,
    ) -> Option<Vec<String>> {
        if start == end {
            return Some(vec![]);
        }
        let mut queue: VecDeque<String> = VecDeque::from(vec![start.into()]);
        // the value of visited is the node from which the key was discovered
        let mut visited: HashMap<String, Option<String>> = HashMap::new();
        visited.insert(start.into(), None);
        let mut depth = 0;
        while 0 < queue.len() && max_depth.map(|max| depth < max).unwrap_or(true) {
            let name = queue.pop_front().unwrap();
            if name == end {
                let mut path = vec![];
                let mut prev = name;
                loop {
                    path.push(prev.clone());
                    prev = match visited.get(&prev) {
                        Some(Some(n)) => n.clone(),
                        Some(None) => {
                            path.reverse();
                            return Some(path);
                        }
                        None => return None,
                    };
                }
            }
            let node = match self.nodes.get(&name) {
                Some(n) => n.borrow(),
                None => continue,
            };
            for outbound in node.outbound.iter() {
                if direction == Direction::Outbound && !visited.contains_key(outbound) {
                    queue.push_back(outbound.clone());
                    visited.insert(outbound.clone(), Some(name.clone()));
                }
            }
            for inbound in node.inbound.iter() {
                if direction == Direction::Inbound && !visited.contains_key(inbound) {
                    queue.push_back(inbound.clone());
                    visited.insert(inbound.clone(), Some(name.clone()));
                }
            }
            depth += 1;
        }
        None
    }
}

fn count_direct_indirect_orbits(graph: &Graph) -> usize {
    let mut count = 0;
    for node in graph.nodes.keys() {
        count += graph.bfs_iter(node, false, Direction::Outbound).count()
    }
    count
}

fn get_orbiting(system: &Graph, orbiter: &str) -> Option<String> {
    system
        .nodes
        .get(orbiter)
        .map(|node| node.borrow())
        .and_then(|node| node.outbound.iter().map(|n| n.clone()).next())
}

fn get_transfers(system: &Graph, start: &str, end: &str) -> Result<usize, String> {
    let possible_path = match (get_orbiting(system, start), get_orbiting(system, end)) {
        (Some(you), Some(san)) => system.traversal(&you, &san, Direction::Any, None),
        _ => {
            return Err(format!(
                "cannot find either \"{}\" or \"{}\" in solar system",
                start, end
            ))
        }
    };
    match possible_path.map(|path| path.len()) {
        Some(length) if length > 0 => Ok(length - 1),
        _ => Err("no path between you and santa".into()),
    }
}

fn main() {
    let input_path = std::env::args().nth(1).expect("input file required");
    let file = std::fs::File::open(input_path).expect("cannot open input file");
    let reader = std::io::BufReader::new(file);
    let mut solar_system = Graph::new();
    for line in reader.lines().filter_map(|res| res.ok()) {
        let split: Vec<_> = line.split(')').collect();
        if let [inner, outer] = split.as_slice() {
            solar_system.add_edge(outer, inner)
        }
    }
    println!("Part 1:");
    println!("orbits: {}", count_direct_indirect_orbits(&solar_system));
    println!("\nPart 2:");
    let transfers = get_transfers(&solar_system, "YOU", "SAN").unwrap();
    println!("transfers: {}", transfers);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_system() -> Graph {
        let mut solar_system = Graph::new();
        solar_system.add_edge("B", "COM");
        solar_system.add_edge("C", "B");
        solar_system.add_edge("D", "C");
        solar_system.add_edge("E", "D");
        solar_system.add_edge("F", "E");
        solar_system.add_edge("G", "B");
        solar_system.add_edge("H", "G");
        solar_system.add_edge("I", "D");
        solar_system.add_edge("J", "E");
        solar_system.add_edge("K", "J");
        solar_system.add_edge("L", "K");
        solar_system
    }

    #[test]
    fn test_orbits_count() {
        let solar_system = test_system();
        assert_eq!(solar_system.outbound_count("D"), 3);
        assert_eq!(solar_system.outbound_count("L"), 7);
        assert_eq!(solar_system.outbound_count("COM"), 0);
        assert_eq!(solar_system.count_edges(), 11);
    }

    //#[test]
    //fn test_graph_outbound_iterator() {
    //    let solar_system = test_system();
    //    let outbound: Vec<_> = solar_system
    //        .bfs_iter("L", false, Direction::Outbound)
    //        .collect();
    //    println!("{:?}", outbound);
    //    println!("{}", outbound.len());
    //    assert!(false);
    //}

    #[test]
    fn test_count_direct_indirect_orbits() {
        let solar_system = test_system();
        assert_eq!(count_direct_indirect_orbits(&solar_system), 42);
    }

    #[test]
    fn test_traversal() {
        let system = test_system();
        let traversal = system.traversal("K", "I", Direction::Any, None);
        println!("{:?}", traversal);
        assert_eq!(
            traversal,
            Some(vec![
                String::from("K"),
                String::from("J"),
                String::from("E"),
                String::from("D"),
                String::from("I"),
            ])
        )
    }

    #[test]
    fn test_get_transfers() {
        let mut system = test_system();
        system.add_edge("YOU", "K");
        system.add_edge("SAN", "I");
        let transfers = get_transfers(&system, "YOU", "SAN");
        assert_eq!(transfers, Ok(4));
    }
}
