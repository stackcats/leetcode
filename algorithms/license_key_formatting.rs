impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut t = Vec::new();
        let mut i = 0;
        for c in s.chars().rev() {
            if c == '-' {
                continue;
            }
            if i == k {
                t.push('-');
                i = 0;
            }
            t.push(c.to_ascii_uppercase());
            i += 1;
        }
        t.iter().rev().collect()
    }
}
