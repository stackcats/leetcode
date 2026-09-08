impl Solution {
    pub fn count_rotations(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                ans += 1;
            }
        }
        if s[0] == s[s.len() - 1] {
            ans += 1;
        }
        if ans == k {
            s.len() as i32 - ans
        } else if ans - 1 == k {
            ans
        } else {
            0
        }
    }
}
