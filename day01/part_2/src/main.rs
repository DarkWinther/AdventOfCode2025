const RANGE: usize = 100;

fn passes_of_zero(first_zero_at: usize, number: usize) -> usize {
    if number < first_zero_at {
        0
    } else {
        1 + (number - first_zero_at) / RANGE
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let mut pointer = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let (str_dir, str_num) = line.split_at(1);
        let direction = str_dir.chars().next().unwrap();
        let number: usize = str_num.parse().unwrap();

        match direction {
            'L' => {
                let first_zero_at = if pointer == 0 { RANGE } else { pointer };
                zero_count += passes_of_zero(first_zero_at, number);
                pointer = (pointer + RANGE - (number % RANGE)) % RANGE;
            }
            'R' => {
                let first_zero_at = if pointer == 0 { RANGE } else { RANGE - pointer };
                zero_count += passes_of_zero(first_zero_at, number);
                pointer = (pointer + (number)).rem_euclid(RANGE);
            }
            _ => panic!("Invalid direction: {}", direction as char),
        }
    }

    println!("Zero count: {}", zero_count);
}
