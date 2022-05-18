use std::collections::HashMap;

fn dfs(g: &HashMap<usize, Vec<usize>>,
       u: usize,
       t: &mut i32,
       visited: &mut Vec<bool>,
       disc: &mut Vec<i32>,
       low: &mut Vec<i32>,
       parents: &mut Vec<usize>,
       bridges: &mut Vec<Vec<i32>>
) {
    visited[u] = true;
    disc[u] = *t;
    low[u] = *t;
    *t += 1;
    for &v in &g[&u] {
        if !visited[v] {
            parents[v] = u;
            dfs(g, v, t, visited, disc, low, parents, bridges);
            low[u] = low[u].min(low[v]);
            if low[v] > disc[u] {
               bridges.push(vec![u as i32, v as i32]);
            }
        } else if v != parents[u] {
            low[u] = low[u].min(disc[v]);
        }
    }
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut g = HashMap::new();
        for conn in connections {
            g.entry(conn[0] as usize).or_insert(vec![]).push(conn[1] as usize);
            g.entry(conn[1] as usize).or_insert(vec![]).push(conn[0] as usize);
        }
        let mut visited = vec![false; n];
        let mut disc = vec![-1; n];
        let mut low = vec![-1; n];
        let mut t = 0;
        let mut parents = vec![n + 1; n];
        let mut ans = Vec::new();
        for u in 0..n {
            if !visited[u as usize] {
                dfs(&g, 0, &mut t, &mut visited, &mut disc, &mut low, &mut parents, &mut ans);
            }
        }
        ans
    }
}
