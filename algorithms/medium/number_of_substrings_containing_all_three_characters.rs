impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut ct = [0; 3];
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        let s = s.as_bytes();
        while r < s.len() {
            ct[(s[r] - b'a') as usize] += 1;
            while ct[0] > 0 && ct[1] > 0 && ct[2] > 0 {
                ans += s.len() - r;
                ct[(s[l] - b'a') as usize] -= 1;
                l += 1;
            }
            r += 1;
        }
        ans as _
    }
}
