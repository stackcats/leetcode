impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() {
            if i + 1 == bits.len() {
                return bits[i] == 0;
            }
            if bits[i] == 1 {
                i += 2;
                continue;
            }
            i += 1;
        }
        false
    }
}
