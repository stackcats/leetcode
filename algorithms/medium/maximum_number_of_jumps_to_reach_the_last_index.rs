impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![i32::MIN; nums.len()];
        dp[0] = 0;
        for j in 1..nums.len() {
            for i in 0..j {
                if (nums[i] - nums[j]).abs() <= target {
                    dp[j] = dp[j].max(dp[i] + 1);
                }
            }
        }

        let &ans = dp.last().unwrap();
        if ans < 1 { -1 } else { ans }
    }
}
