use std::mem;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut a = String::new();
        let mut b = String::new();
        for c in s.chars() {
            if c == 'i' {
                mem::swap(&mut a, &mut b);
            } else {
                a.push(c);
            }
        }

        format!("{}{}", b.chars().rev().collect::<String>(), a)
    }
}
