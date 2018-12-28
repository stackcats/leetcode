// https://leetcode.com/problems/backspace-string-compare/

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut a = Vec::new();
        let mut b = Vec::new();

        for &c in s.as_bytes().iter() {
            if c == b'#' {
                a.pop();
            } else {
                a.push(c);
            }
        }

        for &c in t.as_bytes().iter() {
            if c == b'#' {
                b.pop();
            } else {
                b.push(c);
            }
        }

        if a.len() != b.len() {
            return false;
        }

        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }

        true
    }
}
