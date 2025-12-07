use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let count_rows = grid.len();
    let start_pos = grid[0].iter().position(|&c| c == 'S').unwrap(); // Starting position always in the first row
    let mut count_splits = 0;
    let mut active_paths = HashSet::new();
    active_paths.insert(start_pos);

    for row in 1..count_rows {
        let mut next_paths = HashSet::new();

        for path in active_paths {
            if grid[row][path] == '^' {
                next_paths.insert(path - 1);
                next_paths.insert(path + 1);
                count_splits += 1;
            } else {
                next_paths.insert(path);
            }
        }
        active_paths = next_paths;
    }

    println!("Number of splits: {}", count_splits);
}
