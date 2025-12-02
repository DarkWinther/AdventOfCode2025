use std::time::Instant;

mod repeating_numbers;

// Repeating numbers were pre-generated and saved to a binary file using the code snippet below:
//
// let nums = repeating_numbers::_generate_repeating_numbers();
// repeating_numbers::_save_numbers("part_2_alt/src/repeating_numbers.bin", &nums);

fn main() {
    let start = Instant::now();
    let input = include_str!("../../input.txt");
    let ranges = input.split_terminator(",");
    let repeating_numbers = repeating_numbers::load_numbers("part_2_alt/src/repeating_numbers.bin");

    let sum = ranges
        .map(|range| {
            let (min_str, max_str) = range.split_once("-").unwrap();
            let min = min_str.trim().parse::<usize>().unwrap();
            let max = max_str.trim().parse::<usize>().unwrap();
            let start = repeating_numbers.partition_point(|&x| x < min);
            let end = repeating_numbers.partition_point(|&x| x <= max);

            repeating_numbers[start..end].iter().sum::<usize>()
        })
        .sum::<usize>();

    println!("Sum of invalid IDs: {}", sum);
    println!("Execution time: {:?}", start.elapsed());
}
