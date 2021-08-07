impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut altitude = 0;
        for n in &gain {
            altitude += *n;
            if ans < altitude {
                ans = altitude;
            }
        }
        ans
    }
}
