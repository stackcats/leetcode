// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut dt = HashSet::new();
        for v in nums.iter() {
            if dt.contains(v) {
                return true;
            } else {
                dt.insert(v);
            }
        }

        false
    }
}
