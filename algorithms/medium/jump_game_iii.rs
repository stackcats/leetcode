use std::collections::VecDeque;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut seen = vec![false; arr.len()];
        let mut q = VecDeque::new();
        q.push_back(start);
        while let Some(i) = q.pop_front() {
            if arr[i as usize] == 0 {
                return true;
            }
            seen[i as usize] = true;
            for j in [i - arr[i as usize], i + arr[i as usize]] {
                if (j as usize) < arr.len() && !seen[j as usize] {
                    q.push_back(j);
                }
            }
        }
        false
    }
}
