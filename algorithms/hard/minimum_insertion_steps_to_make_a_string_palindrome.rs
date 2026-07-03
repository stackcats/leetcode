fn lcs(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else if s[i - 1] == t[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[n][m]
}

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let t = s.chars().rev().collect::<String>();
        let n = s.len();
        n as i32 - lcs(s, t)
    }
}
