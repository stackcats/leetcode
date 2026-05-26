impl Solution {
    pub fn password_strength(password: String) -> i32 {
        let mut mp = vec![false; 255];
        let mut ans = 0;
        for &c in password.as_bytes() {
            if mp[c as usize] {
                continue;
            }
            mp[c as usize] = true;
            ans += match c {
                b'a'..=b'z' => 1,
                b'A'..=b'Z' => 2,
                b'0'..=b'9' => 3,
                _ => 5,
            };
        }
        ans
    }
}
