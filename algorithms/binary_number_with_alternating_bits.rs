impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev_bit = 2;
        while n > 0 {
            if prev_bit == n % 2 {
                return false;
            }
            prev_bit = n % 2;
            n /= 2;
        }
        true
    }
}
