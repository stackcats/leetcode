use std::str;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut arr = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < word1.len() && j < word2.len() {
            arr.push(word1[i]);
            arr.push(word2[j]);
            i += 1;
            j += 1;
        }
        while i < word1.len() {
            arr.push(word1[i]);
            i += 1;
        }
        while j < word2.len() {
            arr.push(word2[j]);
            j += 1;
        }
        str::from_utf8(&arr).unwrap().to_string()
    }
}
