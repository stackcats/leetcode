impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut ans = 0;
        let mut carry = 0;
        let s = s.as_bytes();
        for i in (1..s.len()).rev() {
            if (s[i] - b'0' + carry) % 2 == 1 {
                ans += 2;
                carry = 1;
            } else {
                ans += 1;
            }
        }

        ans + carry as i32
    }
}
