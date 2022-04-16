use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut freq = HashMap::new();
        words
            .into_iter()
            .for_each(|w| *freq.entry(w).or_insert(0) += 1);

        let mut h = BinaryHeap::with_capacity(k as usize);
        for (w, f) in freq.into_iter() {
            h.push(Reverse((f, Reverse(w))));

            if h.len() > (k as usize) {
                h.pop();
            }
        }

        let mut ans = Vec::new();
        while !h.is_empty() {
            let Reverse((_, Reverse(w))) = h.pop().unwrap();
            ans.push(w);
        }
        ans.into_iter().rev().collect()
    }
}
