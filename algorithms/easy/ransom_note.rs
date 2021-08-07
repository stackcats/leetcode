use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut m1 = HashMap::new();
        let mut m2 = HashMap::new();
        for c in ransom_note.chars() {
            *m1.entry(c).or_insert(0) += 1;
        }
        for c in magazine.chars() {
            *m2.entry(c).or_insert(0) += 1;
        }
        for (k, v) in m1.iter() {
            if !m2.contains_key(k) {
                return false;
            }
            if v > &m2[k] {
                return false;
            }
        }
        true
    }
}
