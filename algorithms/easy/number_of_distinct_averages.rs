use std::collections::HashSet;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut set = HashSet::new();
        while i < j {
            set.insert(nums[i] + nums[j]);
            i += 1;
            j -= 1;
        }
        set.len() as i32
    }
}
