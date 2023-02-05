use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut hp = BinaryHeap::new();
        gifts.into_iter().for_each(|n| hp.push(n as i64));
        for _ in 0..k {
            let n = hp.pop().unwrap();
            hp.push((n as f64).sqrt() as i64);
        }
        hp.into_iter().sum::<i64>()
    }
}
