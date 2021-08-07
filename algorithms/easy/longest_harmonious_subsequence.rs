// https://leetcode.com/problems/longest-harmonious-subsequence/

use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut dt = HashMap::new();

        let mut ans = 0;
        for n in nums.iter() {
            let ct = dt.entry(n).or_insert(0);
            *ct += 1;

            let c = *ct;
            if let Some(m) = dt.get(&(n + 1)) {
                ans = ans.max(*m + c);
            }

            if let Some(m) = dt.get(&(n - 1)) {
                ans = ans.max(*m + c);
            }
        }

        ans
    }
}
