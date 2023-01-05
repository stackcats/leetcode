impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let mut dp = vec![0usize; 59];
        let mut ans = 1;
        dp[1] = 1;
        for i in 2..=(n as usize) {
            for j in 1..=i / 2 {
                dp[i] = vec![
                    dp[j] * dp[i - j],
                    j * dp[i - j],
                    dp[j] * (i - j),
                    j * (i - j),
                ]
                .iter()
                .max()
                .cloned()
                .unwrap()
                .max(dp[i]);
            }
            ans = ans.max(dp[i]);
        }
        ans as _
    }
}
