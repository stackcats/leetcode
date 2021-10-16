use std::collections::VecDeque;

#[derive(Debug)]
enum Direction {
    N,
    S,
    W,
    E,
}

use Direction::*;

impl Direction {
    fn rev(&self) -> Direction {
        match self {
            N => S,
            S => N,
            W => E,
            E => W,
        }
    }
}

fn can_pass(src: i32, dest: i32, d: Direction) -> bool {
    match src {
        1 => match d {
            N => false,
            S => false,
            W => dest == 1 || dest == 4 || dest == 6,
            E => dest == 1 || dest == 3 || dest == 5,
        },
        2 => match d {
            N => dest == 2 || dest == 3 || dest == 4,
            S => dest == 2 || dest == 5 || dest == 6,
            W => false,
            E => false,
        },
        3 => match d {
            N => false,
            S => dest == 2 || dest == 5 || dest == 6,
            W => dest == 1 || dest == 4 || dest == 6,
            E => false,
        },
        4 => match d {
            N => false,
            S => dest == 2 || dest == 5 || dest == 6,
            W => false,
            E => dest == 1 || dest == 3 || dest == 5,
        },
        5 => match d {
            N => dest == 2 || dest == 3 || dest == 4,
            S => false,
            W => dest == 1 || dest == 4 || dest == 6,
            E => false,
        },
        6 => match d {
            N => dest == 2 || dest == 3 || dest == 4,
            S => false,
            W => false,
            E => dest == 1 || dest == 3 || dest == 5,
        },
        _ => false,
    }
}

fn next_paths(grid: &[Vec<i32>], x: usize, y: usize) -> Vec<(usize, usize, Direction)> {
    match grid[x][y] {
        1 => vec![(x, y + 1, E), (x, y - 1, W)],
        2 => vec![(x - 1, y, N), (x + 1, y, S)],
        3 => vec![(x, y - 1, W), (x + 1, y, S)],
        4 => vec![(x, y + 1, E), (x + 1, y, S)],
        5 => vec![(x - 1, y, N), (x, y - 1, W)],
        6 => vec![(x - 1, y, N), (x, y + 1, E)],
        _ => vec![],
    }
}

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let mut mem = vec![vec![false; grid[0].len()]; grid.len()];

        let mut q = VecDeque::new();
        q.push_back((0, 0));

        while !q.is_empty() {
            let (x, y) = q.pop_front().unwrap();

            if x == grid.len() - 1 && y == grid[0].len() - 1 {
                return true;
            }

            if mem[x][y] {
                continue;
            }

            mem[x][y] = true;

            let paths = next_paths(&grid, x, y);

            for (nx, ny, d) in &paths {
                if *nx < grid.len()
                    && *ny < grid[0].len()
                    && can_pass(grid[*nx][*ny], grid[x][y], d.rev())
                {
                    q.push_back((*nx, *ny));
                }
            }
        }

        false
    }
}
