use std::collections::VecDeque;

impl Solution {
    pub fn path_in_zig_zag_tree(mut label: i32) -> Vec<i32> {
        let mut arr = Vec::new();
        while label > 0 {
            arr.insert(0, label);
            label /= 2;
        }
        let mut ans = vec![1];
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] * 2 {
                ans.push(ans[i - 1] * 2 + 1)
            } else {
                ans.push(ans[i - 1] * 2)
            }
        }
        for i in (0..arr.len()).rev().step_by(2) {
            ans[i] = arr[i];
        }
        ans
    }
}
