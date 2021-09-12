fn dfs(
    arr: &[i32],
    k: usize,
    n: i32,
    curr_sum: i32,
    curr_arr: &mut Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
) {
    if curr_sum == n && k == curr_arr.len() {
        ans.push(curr_arr.clone());
        return;
    }

    if curr_sum > n || k < curr_arr.len() {
        return;
    }

    for i in 0..arr.len() {
        let mut cp = curr_arr.clone();
        cp.push(arr[i]);
        dfs(&arr[i + 1..], k, n, curr_sum + arr[i], &mut cp, ans);
    }
}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let arr: Vec<i32> = (1..=9).collect();
        let mut ans = Vec::new();
        dfs(&arr, k as usize, n, 0, &mut vec![], &mut ans);
        ans
    }
}
