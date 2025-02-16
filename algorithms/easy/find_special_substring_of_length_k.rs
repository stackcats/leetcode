impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let mut l = 0;
        let mut prev = b'0';
        let s = s.as_bytes();
        for i in 0..s.len() {
            if s[i] == prev {
                l += 1;
            } else {
                l = 1;
            }
            if l == k {
                if i + 1 >= s.len() || s[i + 1] != s[i] {
                    return true;
                }
            }
            prev = s[i];
        }
        false
    }
}
