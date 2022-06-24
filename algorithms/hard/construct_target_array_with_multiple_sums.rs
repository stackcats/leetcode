use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        
        let mut pq = BinaryHeap::new();

        let mut sum = 0;
        
        for t in target {
            pq.push(t);
            sum += t;
        }
        
        while pq.peek().unwrap() != &1 {
            let top = pq.pop().unwrap();
            
            if sum - top == 1 {
                break;
            }
            
            let old = top % (sum - top);
            
            sum += old - top;
            
            if old == top || old == 0 {
                return false;
            }

            pq.push(old);
        }
        true
    }
}
