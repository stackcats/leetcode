impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let k = k as i64;
        (n as i64 - k).max(0) * k + (m as i64 - k).max(0) * k
    }
}
