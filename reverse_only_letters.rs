// https://leetcode.com/problems/reverse-only-letters/

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        if s == "" {
            return String::from("");
        }

        let mut ch: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = ch.len() - 1;
        while i < j {
            if !ch[i].is_ascii_alphabetic() {
                i += 1;
                continue;
            }

            if !ch[j].is_ascii_alphabetic() {
                j -= 1;
                continue;
            }

            let t = ch[i];
            ch[i] = ch[j];
            ch[j] = t;
            i += 1;
            j -= 1;
        }

        ch.into_iter().collect()
    }
}
