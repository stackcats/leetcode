use std::collections::VecDeque;

const M: i64 = 1000000000 + 7;

fn mod_pow(mut a: i64, mut b: i64) -> i64 {
    let mut ans = 1;
    while b > 0 {
        if b % 2 == 1 {
            ans = ans * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    ans
}

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = *edges
            .iter()
            .max_by_key(|v| v[1])
            .unwrap()
            .iter()
            .max()
            .unwrap() as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            let (mut u, mut v) = (edge[0] as usize, edge[1] as usize);
            if u > v {
                (u, v) = (v, u);
            }
            g[u - 1].push(v - 1);
        }

        let mut q = VecDeque::new();
        q.push_back(0);
        let mut depth = -1;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let u = q.pop_front().unwrap();
                g[u].iter().for_each(|&v| q.push_back(v));
            }
            depth += 1;
        }

        mod_pow(2, depth - 1) as _
    }
}
