use std::collections::VecDeque;

fn bfs(g: &Vec<Vec<usize>>, n: usize, start: usize) -> Vec<i32> {
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut mp = vec![-1; n];
    while let Some((curr, ct)) = q.pop_front() {
        mp[curr as usize] = ct;
        for &nxt in &g[curr] {
            if mp[nxt] != -1 {
                continue;
            }
            q.push_back((nxt, ct + 1));
        }
    }
    mp
}

impl Solution {
    pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            g[u].push(v);
            g[v].push(u);
        }

        let dx = bfs(&g, n, x as usize);
        let dy = bfs(&g, n, y as usize);
        let dz = bfs(&g, n, z as usize);

        let mut ans = 0;
        for a in 0..n {
            let mut arr = vec![dx[a], dy[a], dz[a]];
            arr.sort();
            if arr[0] * arr[0] + arr[1] * arr[1] == arr[2] * arr[2] {
                ans += 1;
            }
        }
        ans
    }
}
