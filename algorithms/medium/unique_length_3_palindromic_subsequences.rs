use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut l = [usize::MAX; 26];
        let mut r = [0; 26];
        for i in 0..s.len() {
            let ndx = (s[i] - b'a') as usize;
            l[ndx] = l[ndx].min(i);
            r[ndx] = i;
        }

        (b'a'..=b'z').fold(0, |acc, c| {
            let ndx = (c - b'a') as usize;
            if l[ndx] >= r[ndx] {
                return acc;
            }
            acc + s[l[ndx] + 1..r[ndx]]
                .iter()
                .copied()
                .collect::<HashSet<_>>()
                .len()
        }) as _
    }
}
