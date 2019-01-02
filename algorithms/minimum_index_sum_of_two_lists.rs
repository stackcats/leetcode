// https://leetcode.com/problems/minimum-index-sum-of-two-lists/

use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut dt = HashMap::new();
        for (i, v) in list1.iter().enumerate() {
            dt.insert(v, i);
        }

        let mut least_index_sum = usize::max_value();
        let mut ans = Vec::new();

        for (i, v) in list2.iter().enumerate() {
            if dt.contains_key(v) {
                let ndx = i + dt[v];
                if ndx < least_index_sum {
                    least_index_sum = ndx;
                    ans.clear();
                    ans.push(v.clone());
                } else if ndx == least_index_sum {
                    ans.push(v.clone());
                }
            }
        }

        ans
    }
}
