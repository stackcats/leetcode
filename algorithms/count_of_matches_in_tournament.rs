impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
                ans += n;
            } else {
                n = (n - 1) / 2;
                ans += n;
                n += 1;
            }
        }
        ans
    }
}
