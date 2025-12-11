use std::collections::HashMap;

fn count_paths<'a>(
    io: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    target: &str,
    required_index: &HashMap<&str, usize>,
    full_mask: usize,
    mut mask: usize,
    memo: &mut HashMap<(&'a str, usize), usize>,
) -> usize {
    if let Some(&idx) = required_index.get(current) {
        mask |= 1 << idx;
    }

    // Base case
    if current == target {
        return if mask == full_mask { 1 } else { 0 };
    }

    // Memoization lookup
    if let Some(&cached) = memo.get(&(current, mask)) {
        return cached;
    }

    // Recurse
    let total = io
        .get(current)
        .map(|outs| {
            outs.iter()
                .map(|&next| {
                    count_paths(io, next, target, required_index, full_mask, mask, memo)
                })
                .sum()
        })
        .unwrap();

    memo.insert((current, mask), total);
    total
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

    let required = vec!["dac", "fft"];
    let mut required_index = HashMap::new();
    for (i, &node) in required.iter().enumerate() {
        required_index.insert(node, i);
    }
    let full_mask = (1 << required.len()) - 1;

    let paths = count_paths(
        &io,
        "svr",
        "out",
        &required_index,
        full_mask,
        0,
        &mut HashMap::new(),
    );

    println!("Number of possible paths to out: {}", paths);
}
