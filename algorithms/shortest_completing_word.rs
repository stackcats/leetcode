// https://leetcode.com/problems/shortest-completing-word/

use std::collections::HashMap;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut map = HashMap::new();
        let plate = license_plate.to_lowercase();
        for c in plate.chars() {
            if c.is_alphabetic() {
                let ct = map.entry(c).or_insert(0);
                *ct += 1;
            }
        }

        let mut ans = "";

        for word in words.iter() {
            let mut dt = HashMap::new();
            for c in word.chars() {
                let ct = dt.entry(c).or_insert(0);
                *ct += 1;

                let mut flag = true;
                for (k, v) in map.iter() {
                    let ct = dt.entry(*k).or_insert(0);
                    if *ct < *v {
                        flag = false;
                        break;
                    }
                }

                if flag {
                    if ans == "" || ans.len() > word.len() {
                        ans = word;
                    }
                }
            }
        }

        ans.to_string()
    }
}
