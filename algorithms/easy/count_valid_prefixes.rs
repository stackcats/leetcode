impl Solution {
    pub fn count_valid_prefixes(s: String) -> i32 {
        let mut a = 0i32;
        let mut b = 0;
        let mut ans = 0;
        for &x in s.as_bytes() {
            if x == b'0' {
                a += 1;
            } else {
                b += 1;
            }
            if (a - b).abs() <= 1 {
                ans += 1;
            }
        }
        ans
    }
}
