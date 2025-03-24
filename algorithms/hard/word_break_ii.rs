use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut st = HashSet::new();
        for w in word_dict {
            st.insert(w);
        }

        let mut q = VecDeque::new();
        q.push_back((String::new(), s));
        let mut ans = Vec::new();
        while let Some((t, s)) = q.pop_front() {
            if s.is_empty() {
                if !t.is_empty() {
                    ans.push(t);
                }
                continue;
            }

            for i in 1..=s.len() {
                if st.contains(&s[..i]) {
                    let mut v = t.to_string();
                    if !v.is_empty() {
                        v.push(' ');
                    }
                    v.push_str(&s[..i]);
                    q.push_back((v, (&s[i..]).to_string()));
                }
            }
        }
        ans
    }
}
