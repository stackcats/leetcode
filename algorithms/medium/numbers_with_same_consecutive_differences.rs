use std::collections::VecDeque;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut q = VecDeque::new();
        for i in 1..=9 {
            q.push_back((i, 1));
        }
        let mut ans = Vec::new();
        while !q.is_empty() {
            let (x, ct) = q.pop_front().unwrap();
            if ct == n {
                ans.push(x);
                continue;
            }
            let d = x % 10;
            if d + k <= 9 {
                q.push_back((x * 10 + d + k, ct + 1));
            }
            if d - k >= 0 && k != 0 {
                q.push_back((x * 10 + d - k, ct + 1));
            }
        }
        ans
    }
}
