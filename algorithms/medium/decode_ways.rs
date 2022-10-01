impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];
        dp[s.len()] = 1;
        for (i, &c) in s.iter().enumerate().rev() {
            if c == b'0' {
                dp[i] = 0;
                continue;
            }
            dp[i] += dp[i + 1];
            if i < s.len() - 1 {
                if c == b'1' || (c == b'2' && s[i + 1] < b'7') {
                    dp[i] += dp[i + 2];
                }
            }
        }
        dp[0]
    }
}
