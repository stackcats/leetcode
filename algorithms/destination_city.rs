use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut path_map = HashMap::new();
        for path in &paths {
            path_map.insert(&path[0], &path[1]);
        }
        let mut current = &paths[0][0];
        while path_map.contains_key(&current) {
            current = path_map[&current];
        }
        current.to_string()
    }
}
