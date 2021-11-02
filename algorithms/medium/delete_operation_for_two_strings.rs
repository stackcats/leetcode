fn lcs(word1: &str, word2: &str) -> i32 {
    let mut dp = vec![vec![0; word2.len() + 1]; 2];
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let mut bi = 0;
    for i in 1..=word1.len() {
        bi = i & 1;
        for j in 1..=word2.len() {
            if word1[i - 1] == word2[j - 1] {
                dp[bi][j] = dp[1 - bi][j - 1] + 1;
            } else {
                dp[bi][j] = dp[1 - bi][j].max(dp[bi][j - 1]);
            }
        }
    }
    dp[bi][word2.len()]
}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n = lcs(&word1, &word2);
        word1.len() as i32 + word2.len() as i32 - 2 * n
    }
}
