use std::cmp;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1];
        for i in (1..arr.len()).rev() {
            let tmp = cmp::max(arr[i], *ans.last().unwrap());
            ans.push(tmp);
        }
        ans.into_iter().rev().collect()
    }
}
