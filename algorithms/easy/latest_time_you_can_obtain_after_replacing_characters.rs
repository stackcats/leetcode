impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut t = String::new();
        let s = s.as_bytes();
        for i in 0..s.len() {
            if s[i] == b'?' {
                if i == 0 {
                    if s[1] == b'0' || s[1] == b'1' || s[1] == b'?' {
                        t.push('1');
                    } else {
                        t.push('0');
                    }
                } else if i == 1 {
                    if s[0] == b'1' || s[0] == b'?' {
                        t.push('1');
                    } else {
                        t.push('9');
                    }
                } else if i == 3 {
                    t.push('5');
                } else if i == 4 {
                    t.push('9');
                }
            } else {
                t.push(s[i] as char);
            }
        }
        t
    }
}
