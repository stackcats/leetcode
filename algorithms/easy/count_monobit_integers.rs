impl Solution {
    pub fn count_monobit(n: i32) -> i32 {
        let mut ans = 0;
        let mut curr = 0;
        while curr <= n {
            ans += 1;
            curr = curr << 1 | 1;
        }
        ans
    }
}
