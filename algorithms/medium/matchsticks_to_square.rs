impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum: i32 = matchsticks.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let target = sum / 4;
        let mut dp = vec![-1; 1 << matchsticks.len()];
        dp[0] = 0;
        for mask in 0..dp.len() {
            if dp[mask] == -1 {
                continue;
            }

            for (i, m) in matchsticks.iter().enumerate() {
                let j = 1 << i;
                if mask & j == 0 && dp[mask] + m <= target {
                    dp[mask | j] = (dp[mask] + m) % target;
                }
            }
        }
        dp.pop().unwrap() == 0
    }
}
