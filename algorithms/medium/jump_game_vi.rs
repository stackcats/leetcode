use std::collections::BinaryHeap;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        pq.push((nums[0], 0));
        for i in 1..nums.len() {
            while pq.peek().unwrap().1 + (k as usize) < i {
                pq.pop();
            }
            let n = pq.peek().unwrap().0;
            nums[i] += n;
            pq.push((nums[i], i));
        }
        nums.pop().unwrap()
    }
}
