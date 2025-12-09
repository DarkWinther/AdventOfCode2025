use geo::algorithm::covers::Covers;
use geo::{Coord, LineString, Polygon, Rect};
use rayon::prelude::*;

fn main() {
    let input = include_str!("../../input.txt");

    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();

    let polygon = Polygon::new(
        LineString::from(
            points
                .iter()
                .map(|&(x, y)| Coord {
                    x: x as f64,
                    y: y as f64,
                })
                .collect::<Vec<_>>(),
        ),
        vec![],
    );

    let largest_area = points
        .par_iter()
        .enumerate()
        .flat_map(|(i, &p1)| points[i + 1..].par_iter().map(move |&p2| (p1, p2)))
        .filter_map(|((x1, y1), (x2, y2))| {
            let rect = Polygon::from(Rect::new(
                Coord {
                    x: x1 as f64,
                    y: y1 as f64,
                },
                Coord {
                    x: x2 as f64,
                    y: y2 as f64,
                },
            ));
            polygon
                .covers(&rect)
                .then_some((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
        })
        .max()
        .unwrap_or(0);

    println!("Largest area: {}", largest_area);
}
