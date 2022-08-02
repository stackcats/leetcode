use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut pq = BinaryHeap::new();
        for i in 0..k.min(matrix.len()) {
            pq.push(Reverse((matrix[i][0], i, 0)));
        }

        let mut ans = 0;
        for _ in 0..k {
            let Reverse((x, i, j)) = pq.pop().unwrap();
            ans = x;
            if j + 1 < matrix.len() {
                pq.push(Reverse((matrix[i][j+1], i, j + 1)));
            }
        }

        ans
    }
}
