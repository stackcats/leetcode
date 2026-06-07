impl Solution {
    pub fn consecutive_set_bits(n: i32) -> bool {
        let s = format!("{:0b}", n);
        let s = s.as_bytes();
        let mut ct = 0;
        for i in 1..s.len() {
            if s[i] == s[i - 1] && s[i] == b'1' {
                ct += 1;
            }
        }
        ct == 1
    }
}
