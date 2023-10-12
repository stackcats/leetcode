impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut res = Vec::new();
        for w in words {
            for v in w.split(separator) {
                if v != "" {
                    res.push(v.to_string());
                }
            }
        }
        res
    }
}
