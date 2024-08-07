impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut ans = 0;
        let mut curr = 0;
        for c in s.chars() {
            if c == 'E' {
                curr += 1;
                ans = ans.max(curr);
            } else {
                curr -= 1;
            }
        }
        ans
    }
}
