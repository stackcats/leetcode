impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let s = s.as_bytes();
        let (mut cmin, mut cmax) = (0, 0);
        for &c in s {
            if c == b'(' {
                cmin += 1;
                cmax += 1;
            } else if c == b')' {
                cmin -= 1;
                cmax -= 1;
            } else {
                cmax += 1;
                cmin -= 1;
            }

            if cmax < 0 {
                return false;
            }

            cmin = cmin.max(0);
        }

        cmin == 0
    }
}
