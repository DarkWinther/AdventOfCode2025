fn main() {
    let input = include_str!("../../input.txt");

    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = grid.len();
    let width = grid[0].len();

    #[rustfmt::skip]
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0,  -1),          (0,  1),
        (1,  -1), (1,  0), (1,  1),
    ];

    let mut count = 0;

    loop {
        let mut to_remove = vec![];

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == '@' {
                    let mut neighbors = 0;
                    for (dy, dx) in directions.iter() {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;
                        if ny >= 0 && ny < height as isize && nx >= 0 && nx < width as isize {
                            if grid[ny as usize][nx as usize] == '@' {
                                neighbors += 1;
                            }
                        }
                    }
                    if neighbors < 4 {
                        to_remove.push((y, x));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break; // No more cells to remove, exit the loop
        }

        for (y, x) in to_remove {
            grid[y][x] = '.'; // Remove the cell
            count += 1;
        }
    }

    println!("Total cells removed: {}", count);
}
