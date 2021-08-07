// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut st = Vec::new();

        for &b in s.as_bytes().iter() {
            if b == b'(' {
                st.push(b);
            } else if st.len() == 0 || st[st.len() - 1] != b'(' {
                st.push(b);
            } else {
                st.pop();
            }
        }

        st.len() as i32
    }
}
