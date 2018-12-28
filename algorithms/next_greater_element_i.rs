// https://leetcode.com/problems/next-greater-element-i/

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut dt = HashMap::new();
        let mut st = Vec::new();

        for n in nums2.iter() {
            while st.len() > 0 && st[st.len() - 1] < n {
                let m = st.pop().unwrap();
                dt.insert(m, n);
            }
            st.push(n);
        }

        let mut arr: Vec<i32> = Vec::new();
        for n in nums1.iter() {
            let m = dt.entry(n).or_insert(&-1);
            arr.push(**m);
        }

        arr
    }
}
