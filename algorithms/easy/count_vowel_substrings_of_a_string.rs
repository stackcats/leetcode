use std::collections::HashMap;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let mut ans = 0;

        let word: Vec<_> = word.chars().collect();

        let mut m = HashMap::new();
        m.insert('a', -1);
        m.insert('e', -1);
        m.insert('i', -1);
        m.insert('o', -1);
        m.insert('u', -1);

        let mut last_consonant = -1;

        for i in 0..word.len() {
            if let Some(n) = m.get_mut(&word[i]) {
                *n = i as i32;
                let the_most_left_vowel: i32 = *m.values().min().unwrap();
                ans += (the_most_left_vowel - last_consonant).max(0);
            } else {
                last_consonant = i as i32;
            }
        }
        ans
    }
}
