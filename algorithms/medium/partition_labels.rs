use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut map = HashMap::new();
        let s: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            let ndx = map.entry(s[i]).or_insert(0);
            *ndx = i;
        }
        let mut ans = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let mut end = map[&s[i]];
            let mut j = i + 1;
            while j < end {
                if map[&s[j]] > end {
                    end = map[&s[j]];
                }
                j += 1;
            }
            ans.push((end - i + 1) as i32);
            i = end + 1;
        }
        ans
    }
}
