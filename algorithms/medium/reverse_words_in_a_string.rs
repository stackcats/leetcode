impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .filter(|&s| s != "")
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}
