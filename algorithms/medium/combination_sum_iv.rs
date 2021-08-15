use std::collections::HashSet;

fn dfs(nums: &[i32], target: i32, mem: &mut Vec<i32>) -> i32 {
    if 0 == target {
        return 1;
    }

    if target < 0 {
        return 0;
    }

    if mem[target as usize] != -1 {
        return mem[target as usize];
    }
    let mut ct = 0;
    for i in 0..nums.len() {
        ct += dfs(nums, target - nums[i], mem);
    }
    mem[target as usize] = ct;
    return ct;
}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut mem = vec![-1; (target + 1) as usize];
        dfs(&nums, target, &mut mem)
    }
}
