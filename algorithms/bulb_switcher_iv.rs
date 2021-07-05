impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut ans = 0;
        let mut curr = '0';
        for (i, c) in target.chars().enumerate() {
            if (c == curr) {
                continue;
            }
            ans += 1;
            if curr == '0' {
                curr = '1';
            } else {
                curr = '0';
            }
        }
        ans
    }
}
