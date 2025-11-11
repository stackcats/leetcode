impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut ma = nums[0];
        for (i, &n) in nums.iter().enumerate().skip(1) {
            if n > ma {
                ans += (i as i32) * (n - ma);
                ma = n;
            } else if n < ma {
                ans += ma - n;
            }
        }
        ans
    }
}
