use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(mut nums: Vec<i32>) -> i32 {
        let mut st = HashSet::new();

        let mut i = nums.len() - 1;
        while i < nums.len() {
            if st.contains(&nums[i]) {
                break;
            }
            st.insert(nums[i]);
            i -= 1;
        }

        ((i + 1) as f64 / 3.0).ceil() as i32
    }
}
