// https://leetcode.com/problems/score-of-parentheses

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut st = Vec::new();

        for c in s.chars() {
            if c == '(' {
                st.push(-1);
            } else if st[st.len() - 1] == -1 {
                st.pop();
                st.push(1);
            } else {
                let mut ct = 0;
                while st.len() > 0 && st[st.len() - 1] != -1 {
                    ct += st.pop().unwrap();
                }
                st.pop();
                st.push(ct * 2);
            }
        }

        st.iter().fold(0, |acc, x| acc + x)
    }
}
