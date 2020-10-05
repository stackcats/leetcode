use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..arr2.len() {
            map.insert(arr2[i], i);
        }
        arr1.sort_by(|a, b| {
            if map.contains_key(a) && map.contains_key(b) {
                return map[a].cmp(&map[b]);
            }
            if map.contains_key(a) {
                return Ordering::Less;
            }
            if map.contains_key(b) {
                return Ordering::Greater;
            }
            a.cmp(b)
        });
        arr1
    }
}
