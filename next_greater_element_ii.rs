// https://leetcode.com/problems/next-greater-element-ii/

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut st = Vec::new();
        let mut dt = HashMap::new();

        for i in 0..nums.len() {
            while st.len() > 0 && nums[st[st.len() - 1]] < nums[i] {
                let m = st.pop().unwrap();
                dt.insert(m, nums[i]);
            }
            st.push(i);
        }

        for &n in nums.iter() {
            while st.len() > 0 && nums[st[st.len() - 1]] < n {
                let m = st.pop().unwrap();
                dt.insert(m, n);
            }
        }

        let mut arr = Vec::new();
        for i in 0..nums.len() {
            let m = dt.entry(i).or_insert(-1);
            arr.push(*m);
        }

        arr
    }
}
