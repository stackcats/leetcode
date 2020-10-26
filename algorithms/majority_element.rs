use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in &nums {
            let counter = map.entry(*n).or_insert(0);
            *counter += 1;
            if *counter > nums.len() / 2 {
                return *n;
            }
        }
        -1
    }
}
