impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut j = 0;
        for i in 0..s.len() {
            if j == t.len() {
                return 0;
            }

            if s[i] == t[j] {
                j += 1;
            }
        }
        (t.len() - j) as i32
    }
}
