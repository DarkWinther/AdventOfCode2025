fn main() {
    let input = include_str!("../../input.txt");

    let (ranges_str, numbers_str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let numbers = numbers_str
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let count_fresh = numbers
        .iter()
        .filter(|&num| ranges.iter().any(|(min, max)| num >= min && num <= max))
        .count();

    println!("Fresh ingredients count: {}", count_fresh);
}
