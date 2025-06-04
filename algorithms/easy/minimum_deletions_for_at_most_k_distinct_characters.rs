use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut mp = vec![0; 26];
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            mp[i] += 1;
        }

        let mut pq = BinaryHeap::new();
        for i in 0..26 {
            pq.push(Reverse(mp[i]));
        }

        let mut ans = 0;
        while pq.len() > (k as usize) {
            let Reverse(ct) = pq.pop().unwrap();
            ans += ct;
        }

        ans
    }
}
