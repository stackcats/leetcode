fn ones_and_zeros (s: &str) -> (usize, usize) {
    s.as_bytes().iter().fold((0, 0), |(z, o), c| {
        if *c == b'0' {
            (z + 1, o)
        } else {
            (z, o + 1)
        }
    })
}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for s in &strs {
            let (zs, os) = ones_and_zeros(s);
            for i in (zs..=m).rev() {
                for j in (os..=n).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i-zs][j-os]);
                }
            }
        }
        dp[m][n]
    }
}
