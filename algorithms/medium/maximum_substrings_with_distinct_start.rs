impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut ct = [0; 26];
        let mut ans = 0;
        for c in s.chars() {
            let ndx = (c as u8 - b'a') as usize;
            if ct[ndx] == 0 {
                ct[ndx] = 1;
                ans += 1;
            }
        }
        ans
    }
}
