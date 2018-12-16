impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut ct = 0;
        while z > 0 {
            ct += z & 1;
            z >>= 1;
        }
        ct
    }
}
