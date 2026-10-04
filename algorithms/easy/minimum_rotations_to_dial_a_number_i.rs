impl Solution {
    pub fn min_rotations(s: String) -> i32 {
        s.chars().fold((0, '0' as i32), |(ans, prev), c| {
            let c = c as i32;
            let diff = (c - prev).abs();
            (ans + diff.min(10 - diff), c)
        }).0 as _
    }
}
