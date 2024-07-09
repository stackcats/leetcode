impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut t = Vec::new();
        for c in s.chars() {
            if t.len() == 0 {
                t.push(c);
            } else {
                if c.is_digit(10) && t.last().unwrap().is_alphabetic() {
                    t.pop();
                } else {
                    t.push(c)
                }
            }
        }
        t.into_iter().collect()
    }
}
