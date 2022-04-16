impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..s.len() {
            let mut di = 0;
            while (i as i32) - di >= 0 && i + (di as usize) < s.len() && s[i + di as usize] == s[i- di as usize] {
                ans += 1;
                di += 1;
            }
            di = 0;
            while (i as i32) - di >= 0 && i + 1 + (di as usize) < s.len() && s[i + 1 + di as usize] == s[i- di as usize] {
                ans += 1;
                di += 1;
            }
        }
        ans
    }
}
