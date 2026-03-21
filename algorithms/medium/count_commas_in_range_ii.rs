impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut ans = 0;
        let mut base = 999;
        while n > base {
            ans += n - base;
            base = base * 1000 + 999;
        }
        ans
    }
}
