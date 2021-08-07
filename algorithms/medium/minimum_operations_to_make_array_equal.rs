impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let sum = (1 + (n - 1) * 2 + 1) * n / 2;
        sum / 4
    }
}
