impl Solution {
    pub fn score_balance(s: String) -> bool {
        let mut rht = 0;
        for b in s.as_bytes() {
            rht += (b - b'a' + 1) as i32;
        }

        let mut lft = 0;
        for b in s.as_bytes() {
            let s = (b - b'a' + 1) as i32;
            lft += s;
            rht -= s;
            if lft == rht {
                return true;
            }
        }

        false
    }
}
