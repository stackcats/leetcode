impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut all_uppercase = false;
        let mut all_lowercase = true;
        for (i, c) in word.chars().enumerate() {
            if i == 0 && c.is_uppercase() {
                all_uppercase = true;
            } else {
                if c.is_lowercase() {
                    all_uppercase = false;
                } else {
                    all_lowercase = false;
                }
            }
        }
        all_uppercase || all_lowercase
    }
}
