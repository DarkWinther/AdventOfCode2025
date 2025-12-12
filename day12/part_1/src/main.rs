use std::collections::HashSet;

use bitvec::prelude::*;
use rayon::prelude::*;

type Bits = BitVec<u64, Lsb0>; // dynamic bitboard: length = width * height

#[derive(Debug, Clone)]
struct Present {
    id: usize,
    local_mask: u16, // 3x3 mask in 9 least-significant bits (row-major), for speed
    popcnt: u8,      // number of filled cells in the 3x3
}

fn rotate90deg(mask: u16) -> u16 {
    let mut out = 0u16;
    for y in 0..3 {
        for x in 0..3 {
            let bit = y * 3 + x;
            if (mask >> bit) & 1 == 1 {
                let nx = 2 - y;
                let ny = x;
                let nbit = ny * 3 + nx;
                out |= 1 << nbit;
            }
        }
    }
    out
}

fn unique_rotations(mask: u16) -> Vec<u16> {
    let mut seen = HashSet::new();
    let mut rots = Vec::new();
    let mut cur = mask;
    for _ in 0..4 {
        if seen.insert(cur) {
            rots.push(cur);
        }
        cur = rotate90deg(cur);
    }
    rots
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    required: [usize; 6], // required counts for each of the 6 shapes
}

fn bits_len(width: usize, height: usize) -> usize {
    width * height
}

fn cell_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

// Fast overlap: returns true if (a & b) == 0 without allocating
fn disjoint(a: &Bits, b: &Bits) -> bool {
    // Iterate raw storage words directly
    let a_store = a.as_raw_slice();
    let b_store = b.as_raw_slice();
    debug_assert_eq!(a_store.len(), b_store.len());
    for i in 0..a_store.len() {
        if (a_store[i] & b_store[i]) != 0 {
            return false;
        }
    }
    true
}

// In-place OR and AND-NOT without cloning (BitVec supports these efficiently)
fn place(board: &mut Bits, mask: &Bits) {
    *board |= mask;
}

fn unplace(board: &mut Bits, mask: &Bits) {
    *board &= !mask.clone(); // BitVec API requires same-length bitvec; clone is cheap (shares capacity)
}

fn parse_3x3_mask(lines: &[&str]) -> u16 {
    // Row-major into 9 LSBs: bit k = 1 if filled
    assert_eq!(lines.len(), 3);
    let mut m: u16 = 0;
    for y in 0..3 {
        for x in 0..3 {
            let ch = lines[y].chars().nth(x).unwrap();
            if ch == '#' {
                let bit = (y * 3 + x) as u16;
                m |= 1 << bit;
            }
        }
    }
    m
}

fn popcnt_u16(x: u16) -> u8 {
    x.count_ones() as u8
}

fn make_board(width: usize, height: usize) -> Bits {
    bitvec![u64, Lsb0; 0; bits_len(width, height)]
}

fn lift_3x3_to_board(width: usize, height: usize, dx: usize, dy: usize, local_mask: u16) -> Bits {
    let mut mask = make_board(width, height);
    for y in 0..3 {
        for x in 0..3 {
            let bit = (y * 3 + x) as u16;
            if (local_mask >> bit) & 1 == 1 {
                let bx = dx + x;
                let by = dy + y;
                let idx = cell_index(bx, by, width);
                mask.set(idx, true);
            }
        }
    }
    mask
}

fn placements_per_present(region: &Region, presents: &[Present]) -> [Vec<Bits>; 6] {
    let mut out: [Vec<Bits>; 6] = std::array::from_fn(|_| Vec::new());

    for s in presents {
        if region.required[s.id] == 0 {
            continue;
        }

        // get all unique rotations of this present
        let rotations = unique_rotations(s.local_mask);

        for rot in rotations {
            // Slide the 3x3 window
            for dy in 0..=region.height.saturating_sub(3) {
                for dx in 0..=region.width.saturating_sub(3) {
                    let mask = lift_3x3_to_board(region.width, region.height, dx, dy, rot);
                    out[s.id].push(mask);
                }
            }
        }
    }
    out
}

fn min_required_cells(required: &[usize; 6], shapes: &[Present]) -> usize {
    required
        .iter()
        .enumerate()
        .map(|(id, &c)| {
            let pc = shapes[id].popcnt as usize;
            c * pc
        })
        .sum()
}

fn can_fit_region(region: &Region, presents: &[Present]) -> bool {
    // Pre-prune: impossible by area
    // 1. Area bound
    let required_cells = min_required_cells(&region.required, presents);
    if required_cells > (region.width * region.height) {
        return false;
    }

    let placements = placements_per_present(region, presents);

    // 2. Capacity check per shape
    for i in 0..6 {
        if region.required[i] > placements[i].len() {
            return false;
        }
    }

    let mut required = region.required;
    let mut board = make_board(region.width, region.height);

    fn dfs(
        board: &mut Bits,
        required: &mut [usize; 6],
        presents: &[Present],
        per_shape: &[Vec<Bits>; 6],
    ) -> bool {
        // 1. Done?
        if required.iter().all(|&r| r == 0) {
            return true;
        }

        // 2. Area pruning
        let free = board.len() - board.count_ones();
        let need: usize = required
            .iter()
            .enumerate()
            .map(|(id, &c)| c * (presents[id].popcnt as usize))
            .sum();
        if need > free {
            return false;
        }

        // 3. Select most constrained shape
        let mut best_shape = None;
        let mut best_candidates: Vec<&Bits> = Vec::new();

        for i in 0..6 {
            if required[i] == 0 {
                continue;
            }
            let mut candidates = Vec::new();
            for m in &per_shape[i] {
                if disjoint(board, m) {
                    candidates.push(m);
                }
            }
            if candidates.len() < required[i] {
                return false; // impossible to satisfy this shape
            }
            if best_shape.is_none() || candidates.len() < best_candidates.len() {
                best_shape = Some(i);
                best_candidates = candidates;
            }
        }

        let s = best_shape.unwrap();

        // 4. Try placements for shape s
        for &m in &best_candidates {
            place(board, m);
            required[s] -= 1;

            if dfs(board, required, presents, per_shape) {
                return true;
            }

            unplace(board, m);
            required[s] += 1;
        }

        false
    }

    dfs(&mut board, &mut required, presents, &placements)
}

fn parse_presents(lines: &[&str]) -> Vec<Present> {
    let mut presents: Vec<Present> = Vec::with_capacity(6);

    for chunk in lines.chunks(5) {
        let id: usize = chunk[0].trim_end_matches(':').parse().unwrap();

        let mask = parse_3x3_mask(&chunk[1..4]);

        presents.push(Present {
            id,
            local_mask: mask,
            popcnt: popcnt_u16(mask),
        });
    }

    presents
}

fn parse_regions(lines: &[&str]) -> Vec<Region> {
    lines
        .iter()
        .map(|line| {
            let (dimensions, counts) = line.split_once(':').unwrap();
            let (width, height) = dimensions.split_once('x').unwrap();

            // Parse present counts
            let present_count = counts
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut required = [0usize; 6]; // Each region must specify 6 present counts
            required.copy_from_slice(&present_count[..6]);

            Region {
                width: width.trim().parse().unwrap(),
                height: height.trim().parse().unwrap(),
                required,
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("../../input.txt");

    let mut lines = input.lines();
    let present_lines = lines.by_ref().take(30).collect::<Vec<_>>(); // 30 lines = exactly 6 shapes / 1 shape per 5 lines
    let region_lines = lines.collect::<Vec<_>>(); // The rest of the lines
    let presents = parse_presents(&present_lines);
    let regions = parse_regions(&region_lines);

    let sum = regions
        .par_iter()
        .filter(|region| can_fit_region(&region, &presents))
        .count();

    println!("Sum of regions that can fit their shapes: {}", sum);
}
