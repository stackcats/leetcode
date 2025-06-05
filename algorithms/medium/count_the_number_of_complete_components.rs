use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![BTreeSet::new(); n as usize];
        for i in 0..n {
            g[i as usize].insert(i);
        }

        for edge in &edges {
            g[edge[0] as usize].insert(edge[1]);
            g[edge[1] as usize].insert(edge[0]);
        }

        let mut mp = HashMap::new();

        for i in 0..n {
            *mp.entry(g[i as usize].clone()).or_insert(0) += 1;
        }

        mp.iter().filter(|&(k, v)| k.len() == *v).count() as i32
    }
}
