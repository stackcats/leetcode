impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut ones = 0;
        let mut zeros = 0;
        let mut curr_ones = 0;
        let mut curr_zeros = 0;

        for c in s.chars() {
            if c == '1' {
                curr_ones += 1;
                ones = ones.max(curr_ones);
                curr_zeros = 0;
            } else {
                curr_zeros += 1;
                zeros = zeros.max(curr_zeros);
                curr_ones = 0;
            }
        }

        ones > zeros
    }
}
