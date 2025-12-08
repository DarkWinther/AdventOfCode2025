use std::collections::{HashMap, HashSet};

const MAX_CONNECTIONS: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

fn distance(a: Point, b: Point) -> f32 {
    let dx = a.x.abs_diff(b.x) as f32;
    let dy = a.y.abs_diff(b.y) as f32;
    let dz = a.z.abs_diff(b.z) as f32;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn all_pairs(points: &[Point]) -> Vec<((Point, Point), f32)> {
    let mut pairs = vec![];
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let d = distance(points[i], points[j]);
            pairs.push(((points[i], points[j]), d));
        }
    }
    pairs
}

fn build_connections(pairs: &[((Point, Point), f32)]) -> HashMap<Point, Vec<(Point, f32)>> {
    let mut connections: HashMap<Point, Vec<(Point, f32)>> = HashMap::new();

    for &((p1, p2), d) in pairs {
        connections.entry(p1).or_default().push((p2, d));
        connections.entry(p2).or_default().push((p1, d));
    }

    connections
}

fn connected_components(connections: &HashMap<Point, Vec<(Point, f32)>>) -> Vec<Vec<Point>> {
    let mut visited = HashSet::new();
    let mut components = vec![];

    for &node in connections.keys() {
        if !visited.contains(&node) {
            let mut stack = vec![node];
            let mut component = vec![];

            while let Some(current) = stack.pop() {
                if visited.insert(current) {
                    component.push(current);
                    if let Some(neighbors) = connections.get(&current) {
                        for &(next, _) in neighbors {
                            if !visited.contains(&next) {
                                stack.push(next);
                            }
                        }
                    }
                }
            }

            components.push(component);
        }
    }

    components
}

fn main() {
    let input = include_str!("../../input.txt");

    let coords = input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|n| n.parse::<u32>().unwrap());
            Point {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
                z: nums.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut all_possible_pairs = all_pairs(&coords);
    all_possible_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let selected_pairs = &all_possible_pairs[..MAX_CONNECTIONS];

    let connections = build_connections(&selected_pairs);
    let mut chains = connected_components(&connections);

    chains.sort_by(|a, b| b.len().cmp(&a.len()));

    let product = &chains[..3].iter().fold(1, |acc, chain| acc * chain.len());

    println!("Product of sizes of the three largest chains: {}", product);
}
