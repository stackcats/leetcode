impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        for i in 0..=(s.len() / 2) {
            if s[i] == s[n - i - 1] {
                return i as _;
            }
        }
        -1
    }
}
