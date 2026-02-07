impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut ans = 0;
        let mut st = Vec::new();

        for &c in s.as_bytes().into_iter().rev() {
            if let Some(&t) = st.last()
                && c > t
            {
                st.pop();
                ans += 1;
            } else {
                st.push(c);
            }
        }

        ans
    }
}
