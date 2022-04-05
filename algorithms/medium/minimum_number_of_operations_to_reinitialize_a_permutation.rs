impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 1;
        while ans == 0 || i > 1 {
            i = i * 2 % (n - 1);
            ans += 1;
        }
        ans
    }
}
