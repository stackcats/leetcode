impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.bytes()
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as i32 + 1) * ((b'z' - c) as i32 + 1))
            .sum()
    }
}
