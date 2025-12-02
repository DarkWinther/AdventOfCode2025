use rayon::prelude::*;

fn is_repeating_block(n: usize) -> bool {
    // Count digits
    let len = n.checked_ilog10().unwrap_or(0) + 1;

    // Try block sizes
    for block_size in 1..=len / 2 {
        if len % block_size == 0 {
            let pow10 = 10usize.pow(block_size as u32);
            let block_value = n % pow10;
            let mut temp = n;

            let mut matches = true;
            while temp > 0 {
                if temp % pow10 != block_value {
                    matches = false;
                    break;
                }
                temp /= pow10;
            }

            if matches {
                return true;
            }
        }
    }

    false
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
        .filter(|&i| is_repeating_block(i))
        .sum::<usize>();

    println!("Sum of invalid IDs: {}", sum);
}
