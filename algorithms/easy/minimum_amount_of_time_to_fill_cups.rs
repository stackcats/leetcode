use std::collections::BinaryHeap;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        for n in amount {
            if n != 0 {
                pq.push(n);
            }
        }
                
        let mut ans = 0;
        while pq.len() > 1 {
            let a = pq.pop().unwrap();
            let b = pq.pop().unwrap();
            if (a > 1) {
                pq.push(a - 1);
            }
            if (b > 1) {
                pq.push(b - 1);
            }
            ans += 1;
        }
        
        ans + pq.pop().unwrap_or_default()
    }
}
