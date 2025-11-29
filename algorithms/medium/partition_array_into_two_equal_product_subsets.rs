fn dfs(nums: &Vec<i32>, i: usize, p: i64, target: i64) -> bool {
    if i == nums.len() || p > target {
        return false;
    }

    if p == target {
        return true;
    }

    dfs(nums, i + 1, p, target) || dfs(nums, i + 1, p * nums[i] as i64, target)
}

impl Solution {
    pub fn check_equal_partitions(nums: Vec<i32>, target: i64) -> bool {
        let mut p = 1;
        for &n in &nums {
            p *= n as i64;
        }

        if p != target * target {
            return false;
        }

        dfs(&nums, 0, 1, target)
    }
}
