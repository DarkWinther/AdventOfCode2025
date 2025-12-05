fn main() {
    let input = include_str!("../../input.txt");

    let (ranges_str, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    ranges
        .sort_unstable_by(|(min_a, max_a), (min_b, max_b)| min_a.cmp(min_b).then(max_a.cmp(max_b)));

    let merged = ranges.iter().fold(vec![], |mut acc, &(min, max)| {
        if let Some((_, end)) = acc.last_mut() {
            if min <= *end {
                *end = (*end).max(max);
                return acc;
            }
        }
        acc.push((min, max));
        acc
    });

    let distinct_count = merged.iter().map(|(min, max)| max - min + 1).sum::<usize>();

    println!("Distinct ingredients count: {}", distinct_count);
}
