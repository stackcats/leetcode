impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut ans = 0;
        let mut is_started = false;
        for c in s.chars() {
            if c == ' ' {
                if is_started {
                    ans += 1;
                    is_started = false;
                }
            } else {
                if !is_started {
                    is_started = true
                }
            }
        }
        if is_started {
            ans += 1;
        }
        ans
    }
}
