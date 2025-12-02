use rayon::prelude::*;

fn is_repeated_twice(n: usize) -> bool {
    // Handle 0 explicitly
    if n == 0 {
        return false;
    }

    // Count digits
    let len = n.checked_ilog10().unwrap_or(0) + 1;

    // Must be even length
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let pow10 = 10usize.pow(half as u32);

    let left = n / pow10; // first half of digits
    let right = n % pow10; // second half of digits

    left == right
}

fn main() {
    let input = include_str!("../../input.txt");
    let ranges = input.split_terminator(",");

    let sum = ranges
        .par_bridge()
        .map(|range| {
            let (min_str, max_str) = range.split_once("-").unwrap();
            let min = min_str.trim().parse::<usize>().unwrap();
            let max = max_str.trim().parse::<usize>().unwrap();
            (min, max)
        })
        .flat_map(|(min, max)| (min..=max).into_par_iter())
        .filter(|&i| is_repeated_twice(i))
        .sum::<usize>();

    println!("Sum of invalid IDs: {}", sum);
}
