use std::collections::HashMap;

fn count_paths(io: &HashMap<&str, Vec<&str>>, current: &str, target: &str) -> usize {
    if current == target {
        return 1;
    }
    io.get(current)
        .map(|outs| outs.iter().map(|&next| count_paths(io, next, target)).sum())
        .unwrap_or(0)
}

fn main() {
    let input = include_str!("../../input.txt");

    let io = input
        .lines()
        .map(|line| {
            let (i, o) = line.split_once(':').unwrap();
            (i, o.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();

    let paths = count_paths(&io, "you", "out");
    println!("Number of possible paths to out: {}", paths);
}
