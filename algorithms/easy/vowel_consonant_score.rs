impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut v = 0;
        let mut c = 0;

        for each in s.chars() {
            match each {
                'a' | 'e' | 'i' | 'o' | 'u' => v += 1,
                each if each.is_alphabetic() => c += 1,
                _ => {}
            }
        }

        if c > 0 {
            v / c
        } else {
            0
        }
    }
}
