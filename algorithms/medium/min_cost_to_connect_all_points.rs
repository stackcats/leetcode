use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn manhattan(p1: &[i32], p2: &[i32]) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut pool = HashSet::new();
        for i in 0..points.len() {
            pool.insert(i);
        }
        let mut ans = 0;
        heap.push(Reverse((0, 0)));
        while !pool.is_empty() {
            let Reverse((d, to)) = heap.pop().unwrap();
            if !pool.contains(&to) {
                continue;
            }
            pool.remove(&to);
            ans += d;
            for i in pool.iter() {
                let d = manhattan(&points[to], &points[*i]);
                heap.push(Reverse((d, *i)));
            }
        }
        ans
    }
}
