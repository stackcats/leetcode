// https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut h1 = HashMap::new();
        let mut h2 = HashMap::new();

        for c in s.chars() {
            let ct = h1.entry(c).or_insert(0);
            *ct += 1;
        }

        for c in t.chars() {
            let ct = h2.entry(c).or_insert(0);
            *ct += 1;
        }

        h1 == h2
    }
}
