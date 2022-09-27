use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for i in 0..nums.len() - 1 {
            let sum = nums[i] + nums[i + 1];
            if !set.insert(sum) {
                return true;
            }
        }
        false
    }
}
