impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ct = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == 'L' {
                ct += 1;
            } else {
                ct -= 1;
            }
            if ct == 0 {
                ans += 1;
            }
        }
        ans
    }
}
