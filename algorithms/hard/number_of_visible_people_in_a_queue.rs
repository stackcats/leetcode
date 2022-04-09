impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut st = Vec::new();
        let mut ans = Vec::new();

        for h in heights.into_iter().rev() {
            if st.is_empty() {
                st.push(h);
                ans.push(0);
                continue;
            }

            if h < st[st.len() - 1] {
                st.push(h);
                ans.push(1);
                continue;
            }

            let mut n = 0;
            while !st.is_empty() && st[st.len() - 1] < h {
                n += 1;
                st.pop();
            }

            if !st.is_empty() {
                n += 1;
            }

            st.push(h);
            ans.push(n);
        }

        ans.into_iter().rev().collect()
    }
}
