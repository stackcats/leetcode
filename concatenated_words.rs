// https://leetcode.com/problems/concatenated-words/

use std::collections::HashSet;

fn rec(dt: &mut HashSet<&[u8]>, word: &[u8]) -> bool {
    if dt.contains(word) {
        return true;
    }

    for i in 1..word.len() {
        if !dt.contains(&word[..i]) {
            continue;
        }

        if rec(dt, &word[i..]) {
            return true;
        }
    }

    false
}

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let mut dt = HashSet::new();
        for word in words.iter() {
            if word == "" {
                continue;
            }

            dt.insert(word.as_bytes());
        }

        let mut ans = Vec::new();
        for word in words.iter() {
            if word == "" {
                continue;
            }

            let bs = word.as_bytes();
            for i in 1..bs.len() {
                if dt.contains(&bs[..i]) && rec(&mut dt, &bs[i..]) {
                    ans.push(word.to_string());
                    break;
                }
            }
        }

        ans
    }
}
