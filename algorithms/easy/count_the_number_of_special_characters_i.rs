use std::iter::zip;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower = [false; 26];
        let mut upper = [false; 26];
        for c in word.as_bytes() {
            if *c >= b'a' {
                lower[(c - b'a') as usize] = true;
            } else {
                upper[(c - b'A') as usize] = true;
            }
        }
        zip(lower, upper).filter(|(a, b)| *a && *b).count() as i32
    }
}
