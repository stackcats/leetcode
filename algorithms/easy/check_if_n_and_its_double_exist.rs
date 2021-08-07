use std::collections::HashMap;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for i in 0..arr.len() {
            map.insert(arr[i] * 2, i);
        }
        for i in 0..arr.len() {
            if map.contains_key(&arr[i]) && i != map[&arr[i]] {
                return true;
            }
        }
        false
    }
}
