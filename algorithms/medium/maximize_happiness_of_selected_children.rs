use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut hp = BinaryHeap::from(happiness);
        let mut ans = 0;
        for i in 0..k {
            let v = hp.pop().unwrap();
            ans += (v - i).max(0) as i64;
        }
        ans
    }
}
