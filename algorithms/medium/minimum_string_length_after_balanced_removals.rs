impl Solution {
    pub fn min_length_after_removals(s: String) -> i32 {
        let mut ct: i32 = 0;
        for c in s.chars() {
            match c {
                'a' => ct += 1,
                'b' => ct -= 1,
                _ => {}
            }
        }
        ct.abs()
    }
}
