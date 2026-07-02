use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let mut dis = vec![vec![i32::MAX; n]; m];
        dis[0][0] = grid[0][0];

        let mut q = VecDeque::new();
        q.push_back((0, 0));

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        while let Some((i, j)) = q.pop_front() {
            if (i, j) == (m - 1, n - 1) {
                return true;
            }

            for (x, y) in dirs.iter() {
                let ni = ((i as i32) + x) as usize;
                let nj = ((j as i32) + y) as usize;
                if ni >= m || nj >= n {
                    continue;
                }

                let cost = dis[i][j] + grid[ni][nj];
                if cost >= health || cost >= dis[ni][nj] {
                    continue;
                }

                dis[ni][nj] = cost;
                q.push_back((ni, nj));
            }
        }

        false
    }
}
