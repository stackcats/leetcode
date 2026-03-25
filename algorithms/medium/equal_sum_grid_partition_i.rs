impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut q = vec![vec![0; n]; m];
        q[0][0] = grid[0][0] as i64;
        for i in 1..n {
            q[0][i] = grid[0][i] as i64 + q[0][i - 1];
        }
        for i in 1..m {
            q[i][0] = grid[i][0] as i64 + q[i - 1][0];
        }
        for i in 1..m {
            for j in 1..n {
                q[i][j] = grid[i][j] as i64 + q[i - 1][j] + q[i][j - 1] - q[i - 1][j - 1];
            }
        }

        for i in 0..m - 1 {
            if q[i][n - 1] * 2 == q[m - 1][n - 1] {
                return true;
            }
        }

        for j in 0..n - 1 {
            if q[m - 1][j] * 2 == q[m - 1][n - 1] {
                return true;
            }
        }

        false
    }
}
