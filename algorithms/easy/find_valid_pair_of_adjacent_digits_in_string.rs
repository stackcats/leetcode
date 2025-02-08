fn ndx(i: u8) -> usize {
    (i - b'0') as usize
}

impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let mut ct = vec![0; 10];
        let s = s.as_bytes();
        for &b in s {
            ct[ndx(b)] += 1;
        }

        for i in 1..s.len() {
            if s[i] != s[i - 1]
                && ct[ndx(s[i])] == s[i] - b'0'
                && ct[ndx(s[i - 1])] == s[i - 1] - b'0'
            {
                return format!("{}{}", s[i - 1] as char, s[i] as char);
            }
        }

        "".to_string()
    }
}
