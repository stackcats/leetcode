impl Solution {
    pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        let mut ans = 0;
        for (mut n) in nums {
            while n > 0 {
                if n % 10 == digit {
                    ans += 1;
                }
                n /= 10;
            }
        }
        ans
    }
}
