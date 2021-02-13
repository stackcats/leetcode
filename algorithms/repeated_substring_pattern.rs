impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        for len in (1..=(s.len() / 2)).rev() {
            if s.len() % len != 0 {
                continue;
            }
            let mut is_repeated = true;
            for i in len..s.len() {
                if s[i] != s[i - len] {
                    is_repeated = false;
                    break;
                }
            }
            if is_repeated {
                return true;
            }
        }
        false
    }
}
