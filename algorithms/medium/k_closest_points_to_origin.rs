use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut pq = BinaryHeap::with_capacity(k);
        for p in &points {
            let d = p[0] * p[0] + p[1] * p[1];
            pq.push((d, vec![p[0], p[1]]));
            if pq.len() > k {
                pq.pop();
            }
        }
        pq.into_iter().map(|(_, p)| p).collect()
    }
}
