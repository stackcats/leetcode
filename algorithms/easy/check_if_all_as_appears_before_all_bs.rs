impl Solution {
    pub fn check_string(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        while i < s.len() {
            if s[i] == b'b' {
                break;
            }
            i += 1;
        }
        if i == s.len() {
            return true;
        }
        let mut j = (s.len() - 1) as i32;
        while j >= 0 {
            if s[j as usize] == b'a' {
                break;
            }
            j -= 1;
        }
        if j < 0 {
            return true;
        }
        (j as usize) < i
    }
}
