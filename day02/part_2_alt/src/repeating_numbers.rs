use std::{fs::File, io::BufWriter};

use bincode::config;

fn _multiplier(len: usize, repeat: usize) -> usize {
    let mut m = 0;
    for k in 0..repeat {
        m += 10usize.pow((len * k) as u32);
    }
    m
}

pub fn _generate_repeating_numbers() -> Vec<usize> {
    let mut nums = Vec::new();

    // repeat limits per base length
    let limits = vec![
        (1, 10),
        (2, 5),
        (3, 3),
        (4, 2),
        (5, 2),
    ];

    for (len, max_repeat) in limits {
        for repeat in 2..=max_repeat {
            let mult = _multiplier(len, repeat);
            let start = 10usize.pow((len - 1) as u32);
            let end = 10usize.pow(len as u32) - 1;

            for base in start..=end {
                nums.push(base * mult);
            }
        }
    }

    nums.sort_unstable();
    nums.dedup();
    nums
}

pub fn _save_numbers(path: &str, numbers: &[usize]) {
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);
    bincode::encode_into_std_write(numbers, &mut writer, config::standard()).unwrap();
}

pub fn load_numbers(path: &str) -> Vec<usize> {
    let file = File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let numbers: Vec<usize> =
        bincode::decode_from_std_read(&mut reader, config::standard()).unwrap();
    numbers
}
