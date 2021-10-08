use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn power(n: i32, map: &mut HashMap<i32, i32>) -> i32 {
    if map.contains_key(&n) {
        return map[&n];
    }

    let next = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    let ct = power(next, map) + 1;
    map.insert(n, ct);
    ct
}

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut map = HashMap::with_capacity(3000);
        map.insert(1, 0);

        let mut heap = BinaryHeap::with_capacity(k as usize);

        for i in lo..=hi {
            let ct = power(i, &mut map);
            heap.push((ct, i));
            if heap.len() > (k as usize) {
                heap.pop();
            }
        }

        let top = heap.peek().unwrap();
        top.1
    }
}
