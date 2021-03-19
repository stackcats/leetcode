fn is_palindrome(s: &[u8]) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                if is_palindrome(&s[i + 1..=j]) || is_palindrome(&s[i..j]) {
                    return true;
                }
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
