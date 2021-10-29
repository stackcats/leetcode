use std::collections::VecDeque;

const dirs: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        let mut dp = vec![vec![-1; n]; n];

        let mut q = VecDeque::new();
        q.push_back((0, 0, 1));

        while !q.is_empty() {
            let (i, j, steps) = q.pop_front().unwrap();
            if dp[i][j] != -1 {
                continue;
            }

            dp[i][j] = steps;

            dirs.iter().for_each(|(dx, dy)| {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx >= 0
                    && (nx as usize) < n
                    && ny >= 0
                    && (ny as usize) < n
                    && grid[nx as usize][ny as usize] == 0
                    && dp[nx as usize][ny as usize] == -1
                {
                    q.push_back((nx as usize, ny as usize, steps + 1));
                }
            });
        }

        dp[n - 1][n - 1]
    }
}
