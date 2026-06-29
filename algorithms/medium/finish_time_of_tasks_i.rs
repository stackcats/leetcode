fn dfs(u: usize, g: &Vec<Vec<usize>>, base_time: &Vec<i32>) -> i64 {
    if g[u].is_empty() {
        return base_time[u] as i64;
    }

    let mut latest = 0;
    let mut earliest = i64::MAX;
    for &v in &g[u] {
        let t = dfs(v, g, base_time);
        latest = latest.max(t);
        earliest = earliest.min(t);
    }

    2 * latest - earliest + base_time[u] as i64
}

impl Solution {
    pub fn finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            g[u].push(v);
        }

        dfs(0, &g, &base_time)
    }
}
