use std::collections::BinaryHeap;

impl Solution {
    pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
        let mut hp = BinaryHeap::new();
        for i in 0..technique1.len() {
            hp.push((
                technique1[i] - technique2[i],
                technique1[i] as i64,
                technique2[i] as i64,
            ));
        }

        let mut ans = 0;

        for _ in 0..k {
            let (_, a, b) = hp.pop().unwrap();
            ans += a;
        }

        while !hp.is_empty() {
            let (_, a, b) = hp.pop().unwrap();
            ans += a.max(b);
        }

        ans
    }
}
