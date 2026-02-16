use std::collections::HashSet;

impl Solution {
    pub fn centered_subarrays(nums: Vec<i32>) -> i32 {
        let mut st = HashSet::new();
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                st.insert(nums[j]);
                sum += nums[j];
                if st.contains(&sum) {
                    ans += 1;
                }
            }
            st.clear();
        }

        ans
    }
}
