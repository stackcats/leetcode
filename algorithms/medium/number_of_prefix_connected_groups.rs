use std::collections::HashMap;

impl Solution {
    pub fn prefix_connected(words: Vec<String>, k: i32) -> i32 {
        let k = k as usize;
        let mut mp = HashMap::new();
        for word in words {
            if word.len() < k {
                continue;
            }
            *mp.entry(word[..k].to_string()).or_insert(0) += 1;
        }

        mp.into_values().filter(|v| *v > 1).count() as _
    }
}
