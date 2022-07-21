impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let md = 1000000000 + 7;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            dp[i][0] = 1;
            for j in 1..=k {
                dp[i][j] = (dp[i][j-1] + dp[i-1][j]) % md;
                if j >= i {
                    dp[i][j] = (dp[i][j] - dp[i-1][j-i] + md) % md;
                }
            }
        }
        dp[n][k]
    }
}
