impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; 2];
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut bi = 0;
        for i in 1..=text1.len() {
            bi = i & 1;
            for j in 1..=text2.len() {
                if text1[i - 1] == text2[j - 1] {
                    dp[bi][j] = dp[1 - bi][j - 1] + 1;
                } else {
                    dp[bi][j] = dp[1 - bi][j].max(dp[bi][j - 1]);
                }
            }
        }
        dp[bi][text2.len()]
    }
}
