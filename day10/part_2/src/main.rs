use good_lp::{Expression, ProblemVariables, Solution, SolverModel, microlp, variable};
use regex::Regex;

// Linear problem solver (LPS) --- Integer linear programming (ILP)
fn min_presses_ilp(buttons: &[Vec<usize>], target: &[usize]) -> Option<(usize, Vec<usize>)> {
    let mut vars = ProblemVariables::new();
    let x: Vec<_> = (0..buttons.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let mut expr = Expression::from(0);

    // Objective: minimize sum of presses
    for xi in &x {
        expr = expr + xi;
    }

    let mut problem = vars.minimise(expr).using(microlp);

    // Constraints: each index must match target
    for i in 0..target.len() {
        let mut lhs = Expression::from(0);
        for (j, button) in buttons.iter().enumerate() {
            if button.contains(&i) {
                lhs = lhs + &x[j];
            }
        }
        problem = problem.with(lhs.eq(target[i] as f64));
    }

    let solution = problem.solve().ok()?;
    let presses: Vec<usize> = x
        .iter()
        .map(|&vi| solution.value(vi).round() as usize)
        .collect();

    // reconstruct achieved joltage per index
    let mut achieved = vec![0usize; target.len()];
    for (j, button) in buttons.iter().enumerate() {
        for &i in button {
            achieved[i] += presses[j];
        }
    }

    let total: usize = presses.iter().sum();

    Some((total, presses))
}

// fn min_presses_bfs(buttons: &[Vec<usize>], target: &[usize]) -> Option<usize> {
//     let start = vec![0; target.len()];
//     let mut best: HashMap<Vec<usize>, usize> = HashMap::new();
//     let mut queue = VecDeque::new();

//     best.insert(start.clone(), 0);
//     queue.push_back(start);

//     while let Some(state) = queue.pop_front() {
//         let steps = best[&state];
//         if state == target {
//             return Some(steps);
//         }
//         for button in buttons {
//             let mut next = state.clone();
//             for &i in button {
//                 next[i] += 1;
//             }
//             // prune if any component exceeds target
//             if next.iter().zip(target).any(|(a, b)| a > b) {
//                 continue;
//             }
//             if best.get(&next).map_or(true, |&old| steps + 1 < old) {
//                 best.insert(next.clone(), steps + 1);
//                 queue.push_back(next);
//             }
//         }
//     }
//     None
// }

// fn min_presses_dijkstra(buttons: &[Vec<usize>], target: &[usize]) -> Option<usize> {
//     let start = vec![0; target.len()];
//     let mut dist: HashMap<Vec<usize>, usize> = HashMap::new();
//     let mut heap = BinaryHeap::new();

//     dist.insert(start.clone(), 0);
//     heap.push((Reverse(0), start));

//     while let Some((Reverse(steps), state)) = heap.pop() {
//         if state == target {
//             return Some(steps);
//         }
//         if steps > *dist.get(&state).unwrap_or(&usize::MAX) {
//             continue;
//         }
//         for button in buttons {
//             let mut next = state.clone();
//             for &i in button {
//                 next[i] += 1;
//             }
//             if next.iter().zip(target).any(|(a, b)| a > b) {
//                 continue;
//             }
//             let next_steps = steps + 1;
//             if next_steps < *dist.get(&next).unwrap_or(&usize::MAX) {
//                 dist.insert(next.clone(), next_steps);
//                 heap.push((Reverse(next_steps), next));
//             }
//         }
//     }
//     None
// }

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
        .map(|(_, buttons, joltage)| min_presses_ilp(&buttons, &joltage).unwrap().0)
        .sum::<usize>();

    println!("Sum of minimum presses required: {}", sum_of_min_presses);
}
