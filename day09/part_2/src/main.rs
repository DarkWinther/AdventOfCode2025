fn on_boundary(polygon: &[(u64, u64)], (px, py): (u64, u64)) -> bool {
    let n = polygon.len();
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if x1 == x2 {
            // Vertical segment at x = x1, y in [min..=max]
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            if px == x1 && py >= ymin && py <= ymax {
                return true;
            }
        } else if y1 == y2 {
            // Horizontal segment at y = y1, x in [min..=max]
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            if py == y1 && px >= xmin && px <= xmax {
                return true;
            }
        }
    }
    false
}

fn is_inside_polygon(polygon: &[(u64, u64)], (px, py): (u64, u64)) -> bool {
    // Boundary points are considered inside
    if on_boundary(polygon, (px, py)) {
        return true;
    }

    // Horizontal ray to the right; count crossings with vertical edges
    let mut crossings = 0;
    let n = polygon.len();

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if x1 == x2 {
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);

            // Half-open on y to avoid double counting at vertices
            // Count only if the edge is strictly to the right of the point
            if py >= ymin && py < ymax && px < x1 {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

fn rect_area((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
    (x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1)
}

fn main() {
    let input = include_str!("../../input.txt");
    //     let input = "7,1
    // 11,1
    // 11,7
    // 9,7
    // 9,5
    // 2,5
    // 2,3
    // 7,3";

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
            let (x1, y1) = p1;
            let (x2, y2) = p2;

            // println!(
            //     "Checking point {},{} against {},{}. Inside? {}. Area: {}",
            //     x1,
            //     y1,
            //     x2,
            //     y2,
            //     is_inside_polygon(&points, (x1, y2)) && is_inside_polygon(&points, (x2, y1)),
            //     rect_area(p1, p2)
            // );

            // Only need to check implied corners
            if is_inside_polygon(&points, (x1, y2)) && is_inside_polygon(&points, (x2, y1)) {
                let area = rect_area(p1, p2);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    println!("Largest area: {}", largest_area);
}
