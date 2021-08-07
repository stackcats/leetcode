impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        if !s.contains(&part) {
            s
        }
        remove_occurrences(s.replace(&part, ""))
    }
}
