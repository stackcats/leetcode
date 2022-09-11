use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engineers = Vec::new();
        for i in 0..speed.len() {
            engineers.push((efficiency[i] as i64, speed[i] as i64));
        }
        engineers.sort_by(|a, b| b.cmp(a));

        let mut ans = 0;
        let mut total_speed = 0;
        let mut pq: BinaryHeap<Reverse<_>> = BinaryHeap::new();
        for (e, s) in engineers {
            if pq.len() == (k as usize) {
                total_speed -= pq.pop().unwrap().0;
            }
            pq.push(Reverse(s));
            total_speed += s;
            ans = ans.max(total_speed * e);
        }
        (ans % 1000000007) as i32
    }
}
