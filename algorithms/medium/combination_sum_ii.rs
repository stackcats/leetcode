use std::collections::HashSet;

fn dfs(
    candidates: &[i32],
    arr: &mut Vec<i32>,
    ndx: usize,
    target: i32,
    ans: &mut HashSet<Vec<i32>>,
) {
    if 0 == target {
        ans.insert(arr.to_vec());
        return;
    }

    for i in ndx..candidates.len() {
        if target < candidates[i] {
            break;
        }
        if i > ndx && candidates[i] == candidates[i - 1] {
            continue;
        }
        arr.push(candidates[i]);
        dfs(candidates, arr, i + 1, target - candidates[i], ans);
        arr.pop();
    }
}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ans = HashSet::new();
        dfs(&candidates, &mut Vec::new(), 0, target, &mut ans);
        ans.into_iter().collect()
    }
}
