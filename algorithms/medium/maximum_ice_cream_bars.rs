use std::collections::BinaryHeap;
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut sum = 0;
        let mut heap = BinaryHeap::new();
        for c in costs {
            if sum + c <= coins {
                heap.push(c);
                sum += c;
                continue;
            }
            if let Some(&n) = heap.peek() {
                if n > c {
                    heap.pop();
                    heap.push(c);
                    sum = sum - n + c;
                }
            }
        }
        heap.len() as i32
    }
}
