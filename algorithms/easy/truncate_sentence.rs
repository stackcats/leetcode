impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split(" ")
            .take(k as usize)
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
