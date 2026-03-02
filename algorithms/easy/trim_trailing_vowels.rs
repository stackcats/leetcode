impl Solution {
    pub fn trim_trailing_vowels(s: String) -> String {
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => continue,
                _ => return s[..s.len() - i].to_string(),
            }
        }
        "".to_string()
    }
}
