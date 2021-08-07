fn dfs(nums: &mut [i32], ndx: usize, sum: i32) -> i32 {
    if (ndx >= nums.len()) {
        return sum;
    }

    let include_ndx = dfs(nums, ndx + 1, sum ^ nums[ndx]);
    let exclude_ndx = dfs(nums, ndx + 1, sum);
    include_ndx + exclude_ndx
}

impl Solution {
    pub fn subset_xor_sum(mut nums: Vec<i32>) -> i32 {
        dfs(&mut nums, 0, 0);
    }
}
