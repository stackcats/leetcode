impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut ans = 0;
        for p in &patterns {
            if word.find(p).is_some() {
                ans += 1;
            }
        }
        ans
    }
}
