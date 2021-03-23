impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        if s == "1" {
            return true;
        }
        let s = s.as_bytes();
        let ct = 0;
        for i in (1..s.len()).rev() {
            if s[i] == b'1' && s[i - 1] == b'0' {
                return false;
            }
        }
        true
    }
}
