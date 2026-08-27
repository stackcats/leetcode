impl Solution {
    pub fn is_palindromic(s: String) -> bool {
        let s = s.as_bytes();
        let s = s
            .iter()
            .map(|v| format!("{:08b}", *v))
            .collect::<Vec<_>>()
            .join("");
        let s = s.as_bytes();
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
