impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut st = String::new();
        for c in s.chars() {
            if c == '*' {
                st.pop();
            } else {
                st.push(c);
            }
        }
        st
    }
}
