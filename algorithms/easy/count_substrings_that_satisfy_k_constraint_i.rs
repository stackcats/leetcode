impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut ones = 0;
        let mut zeros = 0;
        let mut i = 0;
        for j in 0..s.len() {
            if s[j] == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }

            while zeros > k && ones > k {
                if s[i] == b'0' {
                    zeros -= 1;
                } else {
                    ones -= 1;
                }
                i += 1;
            }
            ans += (j - i + 1) as i32;
        }
        
        ans
    }
}
