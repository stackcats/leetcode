impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;
        for &c in &coins {
            for i in c..=amount {
                dp[i as usize] += dp[(i - c) as usize];
            }
        }
        dp[amount as usize]
    }
}
