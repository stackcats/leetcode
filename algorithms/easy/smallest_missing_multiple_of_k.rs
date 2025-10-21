use std::collections::HashSet;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let st = nums.into_iter().collect::<HashSet<i32>>();
        let mut i = 1;
        loop {
            if !st.contains(&(k * i)) {
                return k * i;
            }
            i += 1;
        }
    }
}
