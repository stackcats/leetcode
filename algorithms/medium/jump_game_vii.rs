impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let (mi, ma) = (min_jump as usize, max_jump as usize);
        let mut dp = vec![0; s.len()];
        let mut pre = vec![0; s.len()];
        dp[0] = 1;
        for i in 0..mi {
            pre[i] = 1;
        }
        let s = s.as_bytes();
        for i in mi..s.len() {
            if s[i] == b'0' {
                pre[i] = pre[i - mi] - if i <= ma { 0 } else { pre[i - ma - 1] };
                if pre[i] > 0 {
                    dp[i] = 1;
                }
            }
            pre[i] = pre[i - 1] + dp[i];
        }
        *dp.last().unwrap() == 1
    }
}
