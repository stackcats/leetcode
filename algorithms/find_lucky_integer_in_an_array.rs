use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map =  HashMap::new();
        for each in &arr {
            let counter = map.entry(*each).or_insert(0);
            *counter += 1;
        }
        for k in map.keys() {
            if *k == map[k] {
                return *k;
            }
        }
        -1
    }
}
