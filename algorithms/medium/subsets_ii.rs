use std::collections::HashSet;

fn dfs(nums: &[i32], ndx: usize, arr: &mut Vec<i32>, ans: &mut HashSet<Vec<i32>>) {
    if ndx >= nums.len() {
        ans.insert(arr.clone());
        return;
    }

    for i in ndx..nums.len() {
        let mut added = arr.clone();
        added.push(nums[i]);
        dfs(nums, i + 1, &mut added, ans);
        dfs(nums, i + 1, arr, ans);
    }
}

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans = HashSet::new();
        let mut arr = Vec::new();
        dfs(&nums, 0, &mut arr, &mut ans);
        ans.into_iter().collect()
    }
}
