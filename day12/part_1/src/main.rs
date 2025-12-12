use std::collections::HashMap;

use good_lp::{Expression, SolverModel, microlp, variables};

#[derive(Debug, Clone)]
struct Present {
    id: usize,
    cells: Vec<(u32, u32)>, // Fixed 3x3. true = '#', false = '.'
}

impl Present {
    fn rotate90(&self) -> Present {
        Present {
            id: self.id,
            cells: self
                .cells
                .iter()
                .map(|&(x, y)| (y, 2 - x)) // rotate in 3x3
                .collect(),
        }
    }

    fn rotations(&self) -> Vec<Present> {
        let mut rots = Vec::new();
        let mut current = self.clone();
        for _ in 0..4 {
            // All possible rotations
            rots.push(current.clone());
            current = current.rotate90();
        }
        rots
    }
}

#[derive(Debug)]
struct Placement {
    present_id: usize,
    cells: Vec<(u32, u32)>, // absolute coordinates in region
}

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    present_count: Vec<u32>, // Always six counts
}

fn parse_presents(lines: &[&str]) -> Vec<Present> {
    let mut presents: Vec<Present> = Vec::with_capacity(6);

    for chunk in lines.chunks(5) {
        let id: usize = chunk[0].trim_end_matches(':').parse().unwrap();
        let mut cells = Vec::with_capacity(3);

        for (row, line) in chunk[1..4].iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push((col as u32, row as u32));
                }
            }
        }

        presents.push(Present { id, cells });
    }

    presents
}

fn parse_regions(lines: &[&str]) -> Vec<Region> {
    lines
        .iter()
        .map(|line| {
            let (dimensions, counts) = line.split_once(':').unwrap();
            let (width, height) = dimensions.split_once('x').unwrap();

            // Parse shape counts
            let present_count = counts
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            assert_eq!(
                present_count.len(),
                6,
                "Each region must specify 6 shape counts"
            );

            Region {
                width: width.trim().parse().unwrap(),
                height: height.trim().parse().unwrap(),
                present_count,
            }
        })
        .collect()
}

fn generate_all_placements(region: &Region, presents: &[Present]) -> Vec<Placement> {
    let mut placements = Vec::new();

    for present in presents {
        for rot in present.rotations() {
            let max_x = region.width.saturating_sub(3);
            let max_y = region.height.saturating_sub(3);
            for dx in 0..=max_x {
                for dy in 0..=max_y {
                    let shifted: Vec<(u32, u32)> = rot
                        .cells
                        .iter()
                        .map(|&(x, y)| ((x + dx) as u32, (y + dy) as u32))
                        .collect();
                    if shifted
                        .iter()
                        .all(|&(x, y)| x < region.width && y < region.height)
                    {
                        placements.push(Placement {
                            present_id: present.id,
                            cells: shifted,
                        });
                    }
                }
            }
        }
    }

    placements
}

// ILP feasibility check
fn can_fit(region: &Region, presents: &[Present]) -> bool {
    let placements = generate_all_placements(region, presents);

    variables! { vars: placement[placements.len()] (binary); }

    // trivial objective: minimise 0
    let mut model = vars.minimise(0).using(microlp);

    // --- Precompute: placements grouped by shape ---
    let mut shape_to_placements: Vec<Vec<usize>> = vec![Vec::new(); presents.len()];
    for (p_index, pl) in placements.iter().enumerate() {
        shape_to_placements[pl.present_id].push(p_index);
    }

    // --- Shape count constraints ---
    for (s, plist) in shape_to_placements.iter().enumerate() {
        let required = region.present_count[s];
        if required == 0 {
            continue;
        } // skip trivial constraints
        let sum: Expression = plist.iter().map(|&p| placement[p]).sum();
        model.add_constraint(sum.eq(required));
    }

    // --- Precompute: cells → placements covering them ---
    let mut cell_to_placements: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    for (p_index, pl) in placements.iter().enumerate() {
        for &cell in &pl.cells {
            cell_to_placements.entry(cell).or_default().push(p_index);
        }
    }

    // --- No overlap constraints ---
    for (_cell, plist) in cell_to_placements {
        let sum: Expression = plist.into_iter().map(|p| placement[p]).sum();
        model.add_constraint(sum.leq(1.0));
    }

    model.solve().is_ok()
}

fn main() {
    let input = include_str!("../../input.txt");
    //     let input = "0:
    // ###
    // ##.
    // ##.

    // 1:
    // ###
    // ##.
    // .##

    // 2:
    // .##
    // ###
    // ##.

    // 3:
    // ##.
    // ###
    // ##.

    // 4:
    // ###
    // #..
    // ###

    // 5:
    // ###
    // .#.
    // ###

    // 4x4: 0 0 0 0 2 0
    // 12x5: 1 0 1 0 2 2
    // 12x5: 1 0 1 0 3 2";

    let mut lines = input.lines();
    let present_lines = lines.by_ref().take(30).collect::<Vec<_>>();
    let region_lines = lines.collect::<Vec<_>>();
    let presents = parse_presents(&present_lines);
    let regions = parse_regions(&region_lines);

    let sum: u32 = regions
        .iter()
        .filter_map(|region| {
            if can_fit(region, &presents) {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    println!("Sum of fitting regions: {}", sum);

    // for (i, region) in regions.iter().enumerate() {
    //     let fits = can_fit(region, &presents);
    //     println!("Can region {} fit? {}", i, fits);
    // }
}
