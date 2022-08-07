impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        // [a, e, i, o, u]
        let mut dp = [1; 5];
        let m = 1000000007;
        for i in 2..=n {
            let mut next_dp = [0; 5];
            next_dp[1] = (next_dp[1] + dp[0]) % m;
            
            next_dp[0] = (next_dp[0] + dp[1]) % m;
            next_dp[2] = (next_dp[2] + dp[1]) % m;
            
            next_dp[0] = (next_dp[0] + dp[2]) % m;
            next_dp[1] = (next_dp[1] + dp[2]) % m;
            next_dp[3] = (next_dp[3] + dp[2]) % m;
            next_dp[4] = (next_dp[4] + dp[2]) % m;

            next_dp[2] = (next_dp[2] + dp[3]) % m;
            next_dp[4] = (next_dp[4] + dp[3]) % m;

            next_dp[0] = (next_dp[0] + dp[4]) % m;
            
            dp = next_dp;
        }
        let mut ans = 0;
        for v in dp {
            ans = (ans + v) % m
        }
        ans
    }
}

