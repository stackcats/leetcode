use std::collections::HashSet;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let s = s.as_bytes();
        let mut st = HashSet::new();
        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                return true;
            }
            let t = (s[i], s[i + 1]);
            if st.contains(&t) {
                return true;
            }
            st.insert((s[i + 1], s[i]));
        }
        false
    }
}
