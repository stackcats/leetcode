impl Solution {
    pub fn bitwise_complement(mut n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut e = 0;
        let mut ans = 0;
        while n > 0 {
            if n % 2 == 0 {
                ans += 2_i32.pow(e)
            }
            n /= 2;
            e += 1;
        }
        ans
    }
}
