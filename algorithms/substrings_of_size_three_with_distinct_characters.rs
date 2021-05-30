use std::collections::HashSet;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }

        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..(s.len() - 2) {
            let st: HashSet<&u8> = s[i..i + 3].iter().collect();
            if st.len() == 3 {
                ans += 1;
            }
        }
        ans
    }
}
