use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut map = HashMap::new();
        for (i, c) in order.chars().enumerate().fold(|(i, c)|)
            map.insert(c, i);
        }
        let mut s: Vec<char> = s.chars().collect();
        s.sort_by_key(|c| map.get(c).unwrap_or(&0));
        s.into_iter().collect()
    }
}
