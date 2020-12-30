impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let sa: Vec<char> = s.chars().collect();
        let ta: Vec<char> = t.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while i < sa.len() && j < ta.len() {
            if sa[i] == ta[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        i == sa.len()
    }
}
