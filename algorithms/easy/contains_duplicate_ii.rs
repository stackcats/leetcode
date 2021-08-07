// https://leetcode.com/problems/contains-duplicate-ii/

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut dt = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(j) = dt.get_mut(v) {
                if *j + k as usize >= i {
                    return true;
                }
            }
            dt.insert(v, i);
        }

        false
    }
}
