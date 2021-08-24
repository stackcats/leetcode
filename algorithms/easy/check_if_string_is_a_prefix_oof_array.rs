impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut t = String::new();
        for w in &words {
            t.push_str(w);
            if t == s {
                return true;
            }
            if t.len() > s.len() {
                return false;
            }
        }

        false
    }
}
