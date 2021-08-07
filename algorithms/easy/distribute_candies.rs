// https://leetcode.com/problems/distribute-candies/

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let mut s = HashSet::new();
        for c in candies.iter() {
            s.insert(c);
        }

        let mut ans = s.len();
        if ans > candies.len() / 2 {
            ans = candies.len() / 2;
        }

        ans as i32
    }
}
