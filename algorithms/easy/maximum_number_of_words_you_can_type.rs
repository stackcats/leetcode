use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let set: HashSet<char> = broken_letters.chars().collect();
        let mut ans = 0;
        for word in text.split_whitespace() {
            let mut has_broken_letter = false;
            for c in word.chars() {
                if set.contains(&c) {
                    has_broken_letter = true;
                    break;
                }
            }
            if !has_broken_letter {
                ans += 1;
            }
        }
        ans
    }
}
