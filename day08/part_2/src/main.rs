use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

// Euclidean distance between two points
fn distance(a: Point, b: Point) -> f32 {
    let dx = a.x.abs_diff(b.x) as f32;
    let dy = a.y.abs_diff(b.y) as f32;
    let dz = a.z.abs_diff(b.z) as f32;
    (dx * dx + dy * dy + dz * dz).sqrt() // Square root not technically needed as it won't affect ordering but kept for correctness
}

// Generate all possible pairs of points with their distances
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

// A.K.A Union-Find data structure
struct DisjointSetUnion {
    root: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl DisjointSetUnion {
    // Create a new DSU with n nodes where each node is its own set
    fn new(n: usize) -> Self {
        DisjointSetUnion {
            root: (0..n).collect(), // Each node is its own root initially
            rank: vec![0; n],       // All trees start at rank 0
            count: n,               // The amount of disjoint sets is n initially
        }
    }

    // Find the root of the set at index i (With path compression)
    fn find_root(&mut self, i: usize) -> usize {
        // If i is not its own root, recursively find the root
        if self.root[i] != i {
            self.root[i] = self.find_root(self.root[i]); // Path compression makes future lookups faster
        }
        self.root[i]
    }

    // Join the sets at index i and j together
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find_root(i);
        let root_j = self.find_root(j);
        if root_i == root_j {
            // i and j are already in the same set
            return false;
        }
        if self.rank[root_i] < self.rank[root_j] {
            self.root[root_i] = root_j; // The larger set at index j absorbs the smaller set at index i
        } else if self.rank[root_i] > self.rank[root_j] {
            self.root[root_j] = root_i; // The larger set at index i absorbs the smaller set at index j
        } else {
            // If ranks are equal, pick the root arbitrarily and increase its rank.
            self.root[root_j] = root_i;
            self.rank[root_i] += 1;
        }
        // Two sets merged into one, so we have one less set in the DSU
        self.count -= 1;
        true
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let coords = input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|n| n.parse::<u32>().unwrap());
            let x = nums.next().unwrap();
            let y = nums.next().unwrap();
            let z = nums.next().unwrap();
            Point { x, y, z }
        })
        .collect::<Vec<_>>();

    let mut all_possible_pairs = all_pairs(&coords);
    all_possible_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut dsu = DisjointSetUnion::new(coords.len());
    // Map points to indices
    let index_map = coords
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect::<HashMap<Point, usize>>();
    let mut last_pair = None;

    for ((p1, p2), _) in all_possible_pairs {
        let i1 = index_map[&p1];
        let i2 = index_map[&p2];

        if dsu.union(i1, i2) {
            // If two sets merged
            last_pair = Some((p1, p2));
            if dsu.count == 1 {
                // If there is only one set left
                break;
            }
        }
    }

    if let Some((p1, p2)) = last_pair {
        println!(
            "Last pair to connect: {:?} <-> {:?}. X coordinates multiplied: {}",
            p1,
            p2,
            p1.x as u64 * p2.x as u64
        );
    }
}
