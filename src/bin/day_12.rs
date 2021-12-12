use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct Node {
    label: String,
    edges: Vec<String>,
}

trait Cave {
    fn is_large(&self) -> bool;
}

impl Cave for String {
    fn is_large(&self) -> bool {
        self == &self.to_uppercase()
    }
}

impl Cave for Node {
    fn is_large(&self) -> bool {
        self.label.is_large()
    }
}

trait PressedForTime {
    fn still_have_time_for_extra_small_cave(&self) -> bool;
}

impl PressedForTime for &[String] {
    fn still_have_time_for_extra_small_cave(&self) -> bool {
        let mut counts: HashMap<&String, u32> = HashMap::new();
        for item in self.iter() {
            if !item.is_large() && item != "start" && item != "end" {
                let value = *counts.get(item).unwrap_or(&0) + 1;
                counts.insert(item, value);
            }
        }
        !counts.values().any(|it| *it > 1)
    }
}

fn parse() -> HashMap<String, Node> {
    let inputs = adventofcode2021::input_lines(12);
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in inputs {
        let parts: Vec<String> = line.split('-').map(|it| it.to_string()).collect();
        assert!(parts.len() == 2);
        if !(nodes.contains_key(&parts[0])) {
            nodes.insert(
                parts[0].clone(),
                Node {
                    label: parts[0].clone(),
                    edges: vec![],
                },
            );
        }
        if !(nodes.contains_key(&parts[1])) {
            nodes.insert(
                parts[1].clone(),
                Node {
                    label: parts[1].clone(),
                    edges: vec![],
                },
            );
        }
        nodes
            .get_mut(&parts[0])
            .unwrap()
            .edges
            .push(parts[1].clone());
        nodes
            .get_mut(&parts[1])
            .unwrap()
            .edges
            .push(parts[0].clone());
    }
    nodes
}

fn traverse(nodes: &HashMap<String, Node>, path: &[String], from: &Node) -> Vec<Vec<String>> {
    let mut output = vec![];
    for next in from.edges.iter() {
        if next.is_large() || !path.contains(next) {
            let mut next_path: Vec<String> = path.into();
            next_path.push(next.clone());
            if next == "end" {
                output.push(next_path);
            } else {
                output.append(&mut traverse(nodes, &next_path, &nodes[next]));
            }
        }
    }
    output
}

fn traverse2(nodes: &HashMap<String, Node>, path: &[String], from: &Node) -> Vec<Vec<String>> {
    let mut output = vec![];
    for next in from.edges.iter() {
        if next.is_large()
            || !path.contains(next)
            || (path.still_have_time_for_extra_small_cave() && next != "end" && next != "start")
        {
            let mut next_path: Vec<String> = path.into();
            next_path.push(next.clone());
            if next == "end" {
                output.push(next_path);
            } else {
                output.append(&mut traverse2(nodes, &next_path, &nodes[next]));
            }
        }
    }
    output
}

fn main() {
    let nodes = parse();
    let paths = traverse(&nodes, &["start".to_string()], &nodes["start"]);
    println!("Part 1: {}", paths.len());
    let paths2 = traverse2(&nodes, &["start".to_string()], &nodes["start"]);
    println!("Part 2: {}", paths2.len());
}
