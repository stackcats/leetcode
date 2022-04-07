impl Solution {
    pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
        let mut ct = 0;
        while start != goal {
            if start % 2 != goal % 2 {
                ct += 1;
            }
            start /= 2;
            goal /= 2;
        }
        ct
    }
}
