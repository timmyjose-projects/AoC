use std::io;

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

fn is_visible(
    grid: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    direction: Direction,
) -> bool {
    match direction {
        Direction::Top => {
            for posx in (0..x).rev() {
                if grid[posx][y] >= grid[x][y] {
                    return false;
                }
            }
            true
        }

        Direction::Bottom => {
            for posx in x + 1..n {
                if grid[posx][y] >= grid[x][y] {
                    return false;
                }
            }
            true
        }

        Direction::Left => {
            for posy in (0..y).rev() {
                if grid[x][posy] >= grid[x][y] {
                    return false;
                }
            }
            true
        }

        Direction::Right => {
            for posy in y + 1..m {
                if grid[x][posy] >= grid[x][y] {
                    return false;
                }
            }
            true
        }
    }
}

fn part1(grid: &Vec<Vec<u32>>, n: usize, m: usize) {
    let mut trees = n * m - (n - 2) * (m - 2);
    for x in 1..n - 1 {
        for y in 1..m - 1 {
            if is_visible(grid, x, y, n, m, Direction::Top)
                || is_visible(grid, x, y, n, m, Direction::Bottom)
                || is_visible(grid, x, y, n, m, Direction::Left)
                || is_visible(grid, x, y, n, m, Direction::Right)
            {
                trees += 1;
            }
        }
    }
    println!("{trees}");
}

fn process(
    grid: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    direction: Direction,
) -> usize {
    match direction {
        Direction::Top => {
            for posx in (0..x).rev() {
                if grid[posx][y] >= grid[x][y] {
                    return x - posx;
                }
            }
            x
        }

        Direction::Bottom => {
            for posx in x + 1..n {
                if grid[posx][y] >= grid[x][y] {
                    return posx - x;
                }
            }
            n - x - 1
        }

        Direction::Left => {
            for posy in (0..y).rev() {
                if grid[x][posy] >= grid[x][y] {
                    return y - posy;
                }
            }
            y
        }

        Direction::Right => {
            for posy in y + 1..m {
                if grid[x][posy] >= grid[x][y] {
                    return posy - y;
                }
            }
            m - y - 1
        }
    }
}

fn part2(grid: &Vec<Vec<u32>>, n: usize, m: usize) {
    let mut scenic_score = 0;

    for x in 1..n - 1 {
        for y in 1..m - 1 {
            let top = process(grid, x, y, n, m, Direction::Top);
            let bottom = process(grid, x, y, n, m, Direction::Bottom);
            let left = process(grid, x, y, n, m, Direction::Left);
            let right = process(grid, x, y, n, m, Direction::Right);

            let score = top * bottom * left * right;

            if score > scenic_score {
                scenic_score = score;
            }
        }
    }

    println!("{scenic_score}");
}

fn main() {
    let grid = io::stdin()
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n = grid.len();
    let m = grid[0].len();

    part1(&grid, n, m);
    part2(&grid, n, m);
}
