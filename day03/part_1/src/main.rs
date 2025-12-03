fn main() {
    let input = include_str!("../../input.txt");

    let sum = input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

            let mut best = 0;
            for i in 0..digits.len() - 1 {
                let value = digits[i] * 10 + digits[i + 1..].iter().max().unwrap();
                if value > best {
                    best = value;
                }
            }
            // println!("Best for line '{}': {}", line, best);

            best
        })
        .sum::<u32>();

    println!("Sum: {}", sum);
}
