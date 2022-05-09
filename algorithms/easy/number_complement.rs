impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        if num == 0 {
            return 1;
        }
        let mut e = 0;
        let mut ans = 0;
        while num > 0 {
            if num % 2 == 0 {
                ans += 2_i32.pow(e)
            }
            num /= 2;
            e += 1;
        }
        ans
    }
}
