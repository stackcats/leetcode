use std::collections::HashMap;

fn check_pattern(word: &[u8], pattern: &[u8]) -> bool {
    let mut forward = HashMap::new();
    let mut backward = HashMap::new();
    for i in 0..word.len() {
        let v = backward.entry(pattern[i]).or_insert(word[i]);
        if *v != word[i] {
            return false;
        }
        let v = forward.entry(word[i]).or_insert(pattern[i]);
        if *v != pattern[i] {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut ans = Vec::new();
        let pattern = pattern.as_bytes();
        for w in &words {
            if w.len() != pattern.len() {
                continue;
            }
            let bs = w.as_bytes();
            if check_pattern(&bs, &pattern) {
                ans.push(w.to_string());
            }
        }
        ans
    }
}
