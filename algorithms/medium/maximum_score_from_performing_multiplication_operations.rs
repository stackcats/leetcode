use std::collections::HashMap;

fn dfs(nums: &[i32], multipliers: &[i32], m: usize, left: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if dp[m][left] > i32::MIN {
        return dp[m][left];
    }

    if m == multipliers.len() {
        return 0;
    }

    let right = nums.len() - (m - left) - 1;
    let pick_left = dfs(nums, multipliers, m + 1, left + 1, dp) + nums[left] * multipliers[m];
    let pick_right = dfs(nums, multipliers, m + 1, left, dp) + nums[right] * multipliers[m];
    let ans = pick_left.max(pick_right);
    dp[m][left] = ans;
    ans
}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut dp = vec![vec![i32::MIN; nums.len() + 5]; multipliers.len() + 5];
        dfs(&nums, &multipliers, 0, 0, &mut dp)
    }
}
