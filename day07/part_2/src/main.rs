use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let count_rows = grid.len();
    let start_pos = grid[0].iter().position(|&c| c == 'S').unwrap(); // Starting position always in the first row

    let mut active_paths = HashMap::new();
    active_paths.insert(start_pos, 1);

    for row in 1..count_rows {
        let mut next_paths = HashMap::new();

        for (&path, &count) in &active_paths {
            if grid[row][path] == '^' {
                *next_paths.entry(path - 1).or_insert(0) += count;
                *next_paths.entry(path + 1).or_insert(0) += count;
            } else {
                *next_paths.entry(path).or_insert(0) += count;
            }
        }
        active_paths = next_paths;
    }

    println!("Number of paths: {}", active_paths.values().sum::<usize>());
}
