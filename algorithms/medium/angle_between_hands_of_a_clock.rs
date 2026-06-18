impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let mut h = (hour % 12) as f64 * 5.;
        let m = minutes as f64;
        h += m / 12.0;
        let ans = ((m - h) * 6.0).abs();
        ans.min(360.0 - ans)
    }
}
