impl Solution {
    pub fn sum_of_good_integers(n: i32, k: i32) -> i32 {
        let mut ans = 0;
        for x in 1.max(n - k)..=n + k {
            if x & n == 0 {
                ans += x;
            }
        }
        ans
    }
}
