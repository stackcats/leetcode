impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut A = 0;
        let mut L = 0;
        for c in s.chars() {
            if c == 'A' {
                A += 1;
                if A > 1 {
                    return false;
                }
                L = 0;
            } else if c == 'L' {
                L += 1;
                if L > 2 {
                    return false;
                }
            } else {
                L = 0;
            }
        }
        true
    }
}
