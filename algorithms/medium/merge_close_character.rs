use std::collections::VecDeque;
impl Solution {
    pub fn merge_characters(s: String, k: i32) -> String {
        let mut q = VecDeque::new();
        let k = k as usize;
        let mut st = vec![false; 26];
        let mut ans = String::new();

        for &c in s.as_bytes() {
            if q.is_empty() {
                q.push_back(c);
                st[(c - b'a') as usize] = true;
                continue;
            }

            if q.len() > k {
                let d = q.pop_front().unwrap();
                ans.push(d as char);
                st[(d - b'a') as usize] = false;
            }

            if st[(c - b'a') as usize] == false {
                q.push_back(c);
                st[(c - b'a') as usize] = true;
            }
        }

        while let Some(c) = q.pop_front() {
            ans.push(c as char);
        }

        ans
    }
}
