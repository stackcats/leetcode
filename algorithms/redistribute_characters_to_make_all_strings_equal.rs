use std::collections::HashMap;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut ct = HashMap::new();
        for word in &words {
            for c in word.chars() {
                *ct.entry(c).or_insert(0usize) += 1;
            }
        }

        for v in ct.values() {
            if v % words.len() != 0 {
                return false;
            }
        }

        true
    }
}
