use std::collections::HashMap;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let arr: Vec<_> = map.values().collect();
        for i in 1..arr.len() {
            if arr[i] != arr[i - 1] {
                return false;
            }
        }
        true
    }
}
