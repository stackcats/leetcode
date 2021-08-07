impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        while i * 7 + j < n {
            ans += i + j + 1;
            j += 1;
            if j >= 7 {
                i += 1;
                j = 0;
            }
        }
        ans
    }
}
