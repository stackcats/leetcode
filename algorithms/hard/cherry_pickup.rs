impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![vec![i32::MIN; n+1]; n+1]; n+1];
        dp[1][1][1] = grid[0][0];
        for x1 in 1..=n {
            for y1 in 1..=n {
                for x2 in 1..=n {
                    let y2 = x1 + y1 - x2;
                    if dp[x1][y1][x2] > 0 || y2 < 1 || y2 > n || grid[x1-1][y1-1] == -1 || grid[x2-1][y2-1] == -1 {
                        continue;
                    }
                    let pre = vec![
                        dp[x1-1][y1][x2-1],
                        dp[x1-1][y1][x2],
                        dp[x1][y1-1][x2-1],
                        dp[x1][y1-1][x2],
                    ].into_iter().max().unwrap();
                    if pre < 0 {
                        continue;
                    }
                    dp[x1][y1][x2] = pre + grid[x1-1][y1-1];
                    if x1 != x2 {
                        dp[x1][y1][x2] += grid[x2-1][y2-1];
                    }
                }
            }
        }
        dp[n][n][n].max(0)
    }
}
