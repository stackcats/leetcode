use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums);
        while set.contains(&original) {
            original *= 2;
        }
        original
    }
}
