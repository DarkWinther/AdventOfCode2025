use rayon::prelude::*;

fn is_repeated_twice(mut n: usize) -> bool {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    let len = digits.len();
    len % 2 == 0 && digits[..len / 2] == digits[len / 2..]
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
