use std::collections::{HashSet, VecDeque};

use regex::Regex;

fn min_presses(n: usize, buttons: &[Vec<usize>], target: &[bool]) -> Option<usize> {
    let start = vec![false; n];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.clone());
    queue.push_back((start, 0));

    while let Some((state, steps)) = queue.pop_front() {
        if &state == target {
            return Some(steps);
        }

        for button in buttons {
            let mut next = state.clone();
            for &i in button {
                next[i] = !next[i];
            }
            if visited.insert(next.clone()) {
                queue.push_back((next, steps + 1));
            }
        }
    }
    None
}

fn parse_indicators(str: &str) -> Vec<bool> {
    str.trim()
        .trim_matches(&['[', ']'])
        .chars()
        .map(|c| c == '#')
        .collect()
}

fn parse_buttons(str: &str) -> Vec<Vec<usize>> {
    str.trim()
        .split_whitespace()
        .map(|button| {
            button
                .trim_matches(&['(', ')'])
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_joltage(str: &str) -> Vec<usize> {
    str.trim()
        .trim_matches(&['{', '}'])
        .split(',')
        .map(|j| j.parse().unwrap())
        .collect()
}

fn main() {
    let input = include_str!("../../input.txt");

    let re = Regex::new(r"(\[[\.#]*?\])((?:\s\((?:\d,?)*\))*)\s(\{(?:\d,?)+\})").unwrap();

    let machines = input.lines().map(|line| {
        let (_, [indicators, buttons, joltage]) = re.captures(line).unwrap().extract();
        (
            parse_indicators(indicators),
            parse_buttons(buttons),
            parse_joltage(joltage),
        )
    });

    let sum_of_min_presses = machines
        .map(|(indicators, buttons, _)| {
            min_presses(indicators.len(), &buttons, &indicators).unwrap()
        })
        .sum::<usize>();

    println!("Sum of minimum presses required: {}", sum_of_min_presses);
}
