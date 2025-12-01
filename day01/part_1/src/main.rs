const RANGE: usize = 100;

fn main() {
    let input = include_str!("../../input.txt");
    let mut pointer: usize = 50;
    let mut zero_count: usize = 0;

    for line in input.lines() {
        let (str_dir, str_num) = line.split_at(1);
        let direction = str_dir.chars().next().unwrap();
        let number: usize = str_num.parse().unwrap();
        let step = number % RANGE;

        match direction {
            'L' => pointer = (pointer + RANGE - step) % RANGE,
            'R' => pointer = (pointer + step) % RANGE,
            _ => panic!("Invalid direction: {}", direction as char),
        }

        if pointer == 0 {
            zero_count += 1;
        }
    }

    println!("Zero count: {}", zero_count);
}
