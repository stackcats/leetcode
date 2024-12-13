use std::cmp::Ordering;
use std::collections::BTreeSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let st: BTreeSet<i32> = nums.into_iter().collect();
        let arr: Vec<i32> = st.into_iter().collect();
        match arr[0].cmp(&k) {
            Ordering::Less => -1,
            Ordering::Equal => arr.len() as i32 - 1,
            _ => arr.len() as i32,
        }
    }
}
