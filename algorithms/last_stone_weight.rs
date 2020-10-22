use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for s in &stones {
            heap.push(*s);
        }
        while heap.len() > 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();
            if a != b {
                heap.push((a - b).abs());
            }
        }

        heap.pop().unwrap_or_default()
    }
}
