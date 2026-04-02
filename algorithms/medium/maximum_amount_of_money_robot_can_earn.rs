impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        let mut dp = vec![vec![vec![i32::MIN; 3]; n]; m];

        dp[0][0][0] = coins[0][0];
        for k in 1..3 {
            dp[0][0][k] = coins[0][0].max(0);
        }

        for j in 1..n {
            dp[0][j][0] = dp[0][j - 1][0] + coins[0][j];
            dp[0][j][1] = (dp[0][j - 1][1] + coins[0][j]).max(dp[0][j - 1][0]);
            dp[0][j][2] = (dp[0][j - 1][2] + coins[0][j]).max(dp[0][j - 1][1]);
        }

        for i in 1..m {
            dp[i][0][0] = dp[i - 1][0][0] + coins[i][0];
            dp[i][0][1] = (dp[i - 1][0][1] + coins[i][0]).max(dp[i - 1][0][0]);
            dp[i][0][2] = (dp[i - 1][0][2] + coins[i][0]).max(dp[i - 1][0][1]);
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j][0] = dp[i - 1][j][0].max(dp[i][j - 1][0]) + coins[i][j];
                dp[i][j][1] = (dp[i - 1][j][1] + coins[i][j])
                    .max(dp[i][j - 1][1] + coins[i][j])
                    .max(dp[i - 1][j][0])
                    .max(dp[i][j - 1][0]);
                dp[i][j][2] = (dp[i - 1][j][2] + coins[i][j])
                    .max(dp[i][j - 1][2] + coins[i][j])
                    .max(dp[i - 1][j][1])
                    .max(dp[i][j - 1][1]);
            }
        }

        dp[m - 1][n - 1][2]
    }
}
