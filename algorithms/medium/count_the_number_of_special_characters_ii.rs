impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut mp = [-1; 256];
        let mut ans = 0;
        for (i, c) in word.chars().enumerate() {
            if c.is_lowercase() {
                mp[c as usize] = i as i32;
            } else if mp[c as usize] == -1 {
                mp[c as usize] = i as i32;
            }
        }

        for c in b'a'..=b'z' {
            if mp[c as usize] != -1 && mp[c as usize] < mp[c as usize - 32] {
                ans += 1;
            }
        }
        ans
    }
}
