fn dfs(hs: &mut Vec<String>, cur: &mut Vec<char>, n: usize, k: usize) {
    if hs.len() >= k {
        return;
    }
    if cur.len() == n {
        hs.push(cur.to_vec().into_iter().collect());
        return;
    }

    for c in &['a', 'b', 'c'] {
        if cur.len() == 0 || *c != cur[cur.len() - 1] {
            cur.push(*c);
            dfs(hs, cur, n, k);
            cur.pop();
        }
    }
}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut hs = Vec::new();
        let mut arr = Vec::new();
        dfs(&mut hs, &mut arr, n as usize, k as usize);
        if let Some(s) = hs.get(k as usize - 1) {
            s.to_string()
        } else {
            "".to_string()
        }
    }
}
