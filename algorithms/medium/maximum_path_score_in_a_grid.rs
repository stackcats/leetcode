impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut k = k as usize;
        let mut dp = vec![vec![vec![i32::MIN; k + 1]; grid[0].len()]; grid.len()];
        dp[0][0][0] = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                for c in 0..=k {
                    if dp[i][j][c] == i32::MIN {
                        continue;
                    }

                    if i + 1 < grid.len() {
                        let score = grid[i + 1][j];
                        let cost = if score == 0 { 0 } else { 1 };
                        if c + cost <= k {
                            dp[i + 1][j][c + cost] =
                                dp[i + 1][j][c + cost].max(dp[i][j][c] + score);
                        }
                    }

                    if j + 1 < grid[0].len() {
                        let score = grid[i][j + 1];
                        let cost = if score == 0 { 0 } else { 1 };
                        if c + cost <= k {
                            dp[i][j + 1][c + cost] =
                                dp[i][j + 1][c + cost].max(dp[i][j][c] + score);
                        }
                    }
                }
            }
        }

        let ans = dp[grid.len() - 1][grid[0].len() - 1].iter().max().unwrap();
        if *ans == i32::MIN { -1 } else { *ans }
    }
}
