fn dfs(parent: &[i32], curr: usize, depth: &mut [i32]) -> i32 {
    if depth[curr] != -1 {
        return depth[curr];
    }
    depth[curr] = dfs(parent, parent[curr] as usize, depth) + 1;
    depth[curr]
}

impl Solution {
    pub fn weighted_sum(parent: Vec<i32>, nums: Vec<i32>) -> i64 {
        let n = parent.len() as usize;
        let mut depth = vec![-1; n];
        depth[0] = 1;
        (0..n).for_each(|i| {
            dfs(&parent, i, &mut depth);
        });
        let height = depth.iter().max().unwrap();
        (0..n).fold(0, |acc, i| {
            acc + nums[i] as i64 * (*height - depth[i] + 1) as i64
        })
    }
}
