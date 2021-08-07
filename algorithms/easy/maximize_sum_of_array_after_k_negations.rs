use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for x in &a {
            heap.push(Reverse(*x));
        }
        for _ in 0..k {
            if let Some(Reverse(x)) = heap.pop() {
                heap.push(Reverse(-x));
            }
        }
        heap.iter().map(|Reverse(x)| x).sum()
    }
}
