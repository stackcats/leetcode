use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut ans = 0;
        let mut s = String::new();
        let mut set = HashSet::new();
        for c in word.chars() {
            if c.is_digit(10) {
                if s == "0" {
                    s = c.to_string();
                } else {
                    s.push(c);
                }
            } else if s.len() > 0 {
                if !set.contains(&s) {
                    ans += 1;
                    set.insert(s.clone());
                }
                s.truncate(0);
            }
        }
        if s.len() > 0 {
            if !set.contains(&s) {
                ans += 1;
            }
        }
        ans
    }
}
