fn rect_area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn main() {
    let input = include_str!("../../input.txt");

    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut largest_area = 0;

    for (i, &p1) in points.iter().enumerate() {
        for &p2 in &points[i + 1..] {
            let area = rect_area(p1, p2);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("Largest area: {}", largest_area);
}
