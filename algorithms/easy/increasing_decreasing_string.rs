use std::collections::HashMap;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut map:HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let counter = map.entry(c).or_insert(0);
            *counter += 1;
        }
        let mut vec = map.keys().cloned().collect::<Vec<_>>();
        vec.sort();
        let mut arr = Vec::new();
        loop {
            for i in 0..vec.len() {
                if arr.len() >= s.len() {
                    break;
                }
                if map[&vec[i]] == 0 {
                    continue;
                }
                arr.push(vec[i]);
                map.insert(vec[i], map[&vec[i]] - 1);
            }
            for i in (0..vec.len()).rev() {
                if arr.len() >= s.len() {
                    break;
                }
                if map[&vec[i]] == 0 {
                    continue;
                }
                arr.push(vec[i]);
                map.insert(vec[i], map[&vec[i]] - 1);
            }
            
            if arr.len() == s.len() {
                break
            }
        }
        arr.iter().collect::<String>()
    }
}
