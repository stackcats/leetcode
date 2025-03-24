use std::collections::{HashSet, VecDeque};
use std::iter::zip;

fn diff1(s: &str, t: &str) -> bool {
    let mut ct = 0;
    for (a, b) in zip(s.chars(), t.chars()) {
        if a != b {
            ct += 1;
        }
    }
    ct == 1
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut q = VecDeque::new();
        q.push_back((1, begin_word));
        let mut seen = HashSet::new();
        while let Some((ct, s)) = q.pop_front() {
            if s == end_word {
                return ct;
            }
            if seen.contains(&s) {
                continue;
            }
            seen.insert(s.to_string());
            for w in &word_list {
                if diff1(w, &s) {
                    q.push_back((ct + 1, w.to_string()));
                }
            }
        }

        0
    }
}
