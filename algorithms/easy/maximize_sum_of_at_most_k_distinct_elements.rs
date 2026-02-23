use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut pq = BinaryHeap::new();
        let mut st = HashSet::new();
        for n in nums {
            if st.contains(&n) {
                continue;
            }
            if pq.len() < (k as usize) {
                pq.push(Reverse(n));
                st.insert(n);
            } else if let Some(&Reverse(m)) = pq.peek()
                && n > m
            {
                pq.pop();
                pq.push(Reverse(n));
                st.insert(n);
            }
        }

        let mut ans = vec![0; pq.len()];
        for i in (0..pq.len()).rev() {
            ans[i] = pq.pop().unwrap().0;
        }
        ans
    }
}
