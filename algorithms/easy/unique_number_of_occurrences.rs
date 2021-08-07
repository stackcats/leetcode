use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map:HashMap<i32, i32> = HashMap::new();
        for a in &arr {
            let counter = map.entry(*a).or_insert(0);
            *counter += 1;
        }
        let vec: Vec<&i32> = map.values().collect();
        let set: HashSet<&i32> = vec.iter().cloned().collect();
        vec.len() == set.len()
    }
}
