fn dfs(nums: &[i32]) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![];
    }
    if nums.len() == 1 {
        return vec![vec![nums[0]]];
    }
    let n = nums[0];
    let mut arr = dfs(&nums[1..]);
    let mut ans = Vec::new();
    for i in 0..arr.len() {
        for j in 0..=arr[i].len() {
            let mut t = arr[i].clone();
            t.insert(j, n);
            ans.push(t);
        }
    }
    ans
}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        dfs(&nums)
    }
}
