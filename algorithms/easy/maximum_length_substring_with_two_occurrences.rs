impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ct = vec![0; 26];
        let mut len = 2;
        let mut i = 0;
        ct[(s[0] - b'a') as usize] = 1;
        for j in 1..s.len() {
            let ndx = (s[j] - b'a') as usize;
            ct[ndx] += 1;
            while ct[ndx] > 2 {
                ct[(s[i] - b'a') as usize] -= 1;
                i += 1;
            }
            len = len.max(j - i + 1);
        }
        len as i32
    }
}
