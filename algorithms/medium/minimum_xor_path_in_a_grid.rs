impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut dp = vec![vec![vec![false; 1024]; m]; n];
        dp[0][0][grid[0][0] as usize] = true;

        for i in 0..n {
            for j in 0..m {
                for k in 0..1024 {
                    if !dp[i][j][k] {
                        continue;
                    }

                    if i + 1 < n {
                        dp[i + 1][j][grid[i + 1][j] as usize ^ k] = true;
                    }
                    if j + 1 < m {
                        dp[i][j + 1][grid[i][j + 1] as usize ^ k] = true;
                    }
                }
            }
        }

        for k in 0..1024 {
            if dp[n - 1][m - 1][k] {
                return k as _;
            }
        }

        unreachable!()
    }
}
