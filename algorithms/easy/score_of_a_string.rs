impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut sum = 0;
        for i in 0..s.len() - 1 {
            sum += (s[i] as i32 - s[i+1] as i32).abs();
        }
        sum
    }
}
