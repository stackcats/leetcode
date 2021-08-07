use std::collections::HashSet;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        for c in s1 {
            set1.insert(*c);
        }
        for c in s2 {
            set2.insert(*c);
        }
        if set1 != set2 {
            return false;
        }
        let mut ct = 0;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                ct += 1;
            }
        }
        ct == 2
    }
}
