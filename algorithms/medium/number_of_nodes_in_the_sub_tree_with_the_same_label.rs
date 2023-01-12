use std::collections::HashMap;

fn dfs(
    mp: &HashMap<i32, Vec<i32>>,
    curr: i32,
    parent: i32,
    labels: &[u8],
    ans: &mut Vec<i32>,
) -> Vec<i32> {
    let mut ct = vec![0; 26];
    if let Some(childs) = mp.get(&curr) {
        for &c in childs {
            if c == parent {
                continue;
            }
            let s = dfs(mp, c, curr, labels, ans);
            for i in 0..s.len() {
                ct[i] += s[i];
            }
        }
    }
    let ndx = (labels[curr as usize] - b'a') as usize;
    ct[ndx] += 1;
    ans[curr as usize] = ct[ndx];
    ct
}
impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let mut mp = HashMap::new();
        for edge in edges {
            mp.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            mp.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }
        let mut ans = vec![0; n as usize];
        dfs(&mp, 0, 0, labels.as_bytes(), &mut ans);
        ans
    }
}
