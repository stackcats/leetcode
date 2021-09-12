use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Pair(i32, i32);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.0, self.1) == (other.0, other.1)
    }
}

impl Eq for Pair {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        nums.iter().for_each(|n| *map.entry(*n).or_insert(0) += 1);

        let mut heap = BinaryHeap::with_capacity(k as usize);
        for (&n, &v) in &map {
            if heap.len() < (k as usize) {
                heap.push(Reverse(Pair(n, v)));
            } else {
                let Reverse(Pair(_, vp)) = heap.peek().unwrap();
                if v > *vp {
                    heap.pop();
                    heap.push(Reverse(Pair(n, v)));
                }
            }
        }

        heap.into_iter().map(|Reverse(Pair(n, _))| n).collect()
    }
}
