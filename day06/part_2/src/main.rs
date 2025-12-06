fn main() {
    let input = include_str!("../../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid.first().unwrap().len();

    let mut columns = Vec::with_capacity(width);

    let mut col = 0;
    let mut current_col = Vec::with_capacity(height);

    while col < width {
        let is_empty = (0..height).all(|row| grid[row][col].is_whitespace());
        if is_empty {
            columns.push(current_col);
            current_col = Vec::with_capacity(height);
            col += 1;
            continue;
        }

        let mut digits = String::with_capacity(height - 1);

        for row in 0..height - 1 {
            let ch = grid[row][col];
            if !ch.is_whitespace() {
                digits.push(ch);
            }
        }
        current_col.push(digits);

        let operator = grid[height - 1][col];
        if !operator.is_whitespace() {
            current_col.push(operator.to_string());
        }

        col += 1;
    }

    columns.push(current_col);

    let total = columns
        .iter()
        .map(|col| {
            let operator = col
                .iter()
                .find(|s| !s.chars().all(|c| c.is_ascii_digit()))
                .unwrap();
            let numbers = col
                .iter()
                .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            match operator.as_str() {
                "+" => numbers.iter().sum::<u64>(),
                "*" => numbers.iter().product::<u64>(),
                _ => panic!("Unknown operator"),
            }
        })
        .sum::<u64>();

    println!("Total: {}", total);
}
