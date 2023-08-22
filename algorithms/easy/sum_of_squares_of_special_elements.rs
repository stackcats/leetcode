impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let len = nums.len();
        for (i, n) in nums.into_iter().enumerate() {
            if len % (i + 1) == 0 {
                ans += n * n;
            }
        }
        ans
    }
}
