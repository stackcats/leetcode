impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut m = 1;
        while n >= m {
            m <<= 1;
        }
        m - 1
    }
}
