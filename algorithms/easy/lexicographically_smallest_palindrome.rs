impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut s = s.as_bytes().to_vec();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            let m = s[i].min(s[j]);
            s[i] = m;
            s[j] = m;
            i += 1;
            j -= 1;
        }
        String::from_utf8(s).unwrap()
    }
}
