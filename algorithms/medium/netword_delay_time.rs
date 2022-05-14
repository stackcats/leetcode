use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>,  n: i32, k: i32) -> i32 {
        let mut map = HashMap::new();
        for i in 1..=n {
            map.insert(i, vec![]);
        }
        for time in times {            
            map.entry(time[0]).and_modify(|arr| arr.push((time[1], time[2])));
        }
        
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, k)));
        
        let mut ans = 0;
        
        while let Some(Reverse((t, s))) = q.pop() {            
            if !map.contains_key(&s) {
                continue;                
            }
            
            ans = ans.max(t);
            
            let arr = map.remove(&s).unwrap();
            for (v, w) in arr {
                q.push(Reverse((w + t, v)));
            }
        }
        
        if map.is_empty() { ans } else { -1 }
    }
}
