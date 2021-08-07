fn dfs(n: i32, k: i32, ndx: i32, curr: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
    if 0 == k {
        ans.push(curr.to_vec());
        return;
    }
    for i in ndx..=(n - k + 1) {
        curr.push(i);
        dfs(n, k - 1, i + 1, curr, ans);
        curr.pop();
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut curr = Vec::new();
        dfs(n, k, 1, &mut curr, &mut ans);
        ans.into_iter().collect()
    }
}
