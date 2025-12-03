fn largest_lexicographic_numeral(line: &str, k: usize) -> u64 {
    let digits: Vec<char> = line.chars().collect();
    let mut stack: Vec<char> = Vec::with_capacity(k);
    let line_len = digits.len();

    for (i, &d) in digits.iter().enumerate() {
        // While we can pop and still have enough digits left
        while !stack.is_empty()
            && stack.last().unwrap() < &d
            && stack.len() - 1 + (line_len - i) >= k
        {
            stack.pop();
        }
        if stack.len() < k {
            stack.push(d);
        }
    }

    stack
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |acc, d| acc * 10 + d as u64)
}

fn main() {
    let input = include_str!("../../input.txt");

    let sum = input
        .lines()
        .map(|line| {
            let best = largest_lexicographic_numeral(line, 12);
            // println!("Best for line '{}': {}", line, best);
            best
        })
        .sum::<u64>();

    println!("Sum: {}", sum);
}
