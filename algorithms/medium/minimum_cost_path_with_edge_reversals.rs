use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![]; n as usize];
        for edges in edges {
            let (u, v, c) = (edges[0], edges[1], edges[2]);
            g[u as usize].push((v, c));
            g[v as usize].push((u, 2 * c));
        }

        let mut pq = BinaryHeap::new();
        for &(v, c) in &g[0] {
            pq.push(Reverse((c, v)));
        }

        let mut seen = HashMap::new();
        seen.insert(0, 0);

        while let Some(Reverse((total, u))) = pq.pop() {
            if let Some(&v) = seen.get(&u)
                && total >= v
            {
                continue;
            }

            seen.insert(u, total);

            for &(v, c) in &g[u as usize] {
                pq.push(Reverse((c + total, v)));
            }
        }

        seen.get(&(n - 1)).cloned().unwrap_or(-1)
    }
}
