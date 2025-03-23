impl Solution {
    pub fn last_remaining(mut n: i32) -> i32 {
        let mut ans = 1;
        let mut step = 1;
        let mut is_left = true;
        while n > 1 {
            if is_left || n % 2 == 1 {
                ans += step;
            }
            step *= 2;
            n /= 2;
            is_left = !is_left;
        }
        ans
    }
}
