use std::collections::VecDeque;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();

        let mut outgoings = vec![0; n];
        let mut rev = vec![vec![]; n];
        let mut q = VecDeque::new();

        for (u, edges) in graph.iter().enumerate() {
            outgoings[u] = edges.len();

            if edges.is_empty() {
                q.push_back(u);
            } else {
                edges.iter().for_each(|&v| rev[v as usize].push(u));
            }
        }

        while let Some(u) = q.pop_front() {
            for &v in &rev[u] {
                outgoings[v] -= 1;
                if outgoings[v] == 0 {
                    q.push_back(v);
                }
            }
        }

        (0..n)
            .filter(|&u| outgoings[u] == 0)
            .map(|u| u as i32)
            .collect()
    }
}
