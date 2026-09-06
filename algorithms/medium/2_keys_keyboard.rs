impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut d = 2;
        while d <= n {
            if n % d == 0 {
                ans += d;
                n /= d;
            } else {
                d += 1;
            }
        }

        ans
    }
}
