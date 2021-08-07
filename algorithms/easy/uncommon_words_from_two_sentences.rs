// https://leetcode.com/problems/uncommon-words-from-two-sentences/

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
        let mut dt = HashMap::new();
        for s in a.split(' ').collect::<Vec<&str>>().iter() {
            let ct = dt.entry(s.to_string()).or_insert(0);
            *ct += 1;
        }

        for s in b.split(' ').collect::<Vec<&str>>().iter() {
            let ct = dt.entry(s.to_string()).or_insert(0);
            *ct += 1;
        }

        let mut ans = Vec::new();
        for s in dt.keys() {
            if dt.get(s).unwrap() == &1 {
                ans.push(s.to_string());
            }
        }

        ans
    }
}
