fn main() {
    let input = include_str!("../../input.txt");

    let rows = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let num_columns = rows.first().unwrap().len();
    let columns: Vec<Vec<&str>> = rows.iter().fold(
        vec![Vec::with_capacity(rows.len()); num_columns],
        |mut acc, row| {
            for (i, val) in row.into_iter().enumerate() {
                acc[i].push(val);
            }
            acc
        },
    );

    let sum = columns
        .iter()
        .map(|col| {
            let &operator = col.last().unwrap();
            let numbers = col[..col.len() - 1]
                .iter()
                .map(|s| s.parse::<u64>().unwrap());
            // println!("Operator: {}, Numbers: {:?}", operator, numbers);

            match operator {
                "*" => numbers.product::<u64>(),
                "+" => numbers.sum::<u64>(),
                _ => panic!("Unknown operator: {}", operator),
            }
        })
        .sum::<u64>();

    println!("Sum: {}", sum);
}
