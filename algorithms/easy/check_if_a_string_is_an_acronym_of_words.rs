impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        if words.len() != s.len() {
            return false;
        }

        let s = s.as_bytes();
        for i in 0..words.len() {
            if words[i].as_bytes()[0] != s[i] {
                return false;
            }
        }

        true
    }
}
