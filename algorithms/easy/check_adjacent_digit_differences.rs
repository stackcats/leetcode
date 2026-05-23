impl Solution {
    pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
        let s = s.as_bytes();
        for i in 1..s.len() {
            if (s[i] as i32 - s[i - 1] as i32).abs() > 2 {
                return false;
            }
        }
        true
    }
}
