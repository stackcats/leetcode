impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }
        
        let mut is_vowel = false;
        let mut is_consonant = false;

        for c in word.chars() {
            if c.is_numeric() {
                continue;
            }

            if !c.is_alphabetic() {
                return false;
            }

            if "aeiou".contains(&c.to_lowercase().to_string()) {
                is_vowel = true;
            } else {
                is_consonant = true;
            }
        }

        is_vowel && is_consonant
    }
}
