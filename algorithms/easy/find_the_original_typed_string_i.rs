impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut ct = word.len();
        let word = word.as_bytes();
        for i in 1..word.len() {
            if word[i] != word[i - 1] {
                ct -= 1;
            }
        }
        ct as i32
    }
}
