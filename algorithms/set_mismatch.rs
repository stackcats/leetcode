// https://leetcode.com/problems/set-mismatch/

use std::collections::HashMap;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut dt = HashMap::new();
        for v in nums.iter() {
            let ct = dt.entry(v).or_insert(0);
            *ct += 1;
        }

        let mut dup = 0;
        let mut miss = 0;
        let end = nums.len() as i32;
        for i in 1..=end {
            if let Some(v) = dt.get(&i) {
                if *v == 2 {
                    dup = i;
                }
            } else {
                miss = i;
            }
        }

        vec![dup, miss]
    }
}
