fn dfs(
    candidates: &[i32],
    arr: &mut Vec<i32>,
    ndx: usize,
    sum: i32,
    target: i32,
    ans: &mut Vec<Vec<i32>>,
) {
    if sum == target {
        ans.push(arr.to_vec());
        return;
    }

    if sum > target {
        return;
    }
    for i in ndx..candidates.len() {
        let mut arr2 = arr.clone();
        arr2.push(candidates[i]);
        dfs(candidates, &mut arr2, i, sum + candidates[i], target, ans);
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        dfs(&candidates, &mut Vec::new(), 0, 0, target, &mut ans);
        ans
    }
}
