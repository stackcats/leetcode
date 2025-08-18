impl Solution {
    pub fn min_sensors(n: i32, m: i32, k: i32) -> i32 {
        let len = (2 * k + 1) as f64;
        (((n as f64) / len).ceil() * ((m as f64) / len).ceil()) as i32
    }
}
