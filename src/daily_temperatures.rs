// https://leetcode.com/problems/daily-temperatures/

impl Solution {
    pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
        let mut st = Vec::new();

        let mut ans = vec![0; t.len()];
        for i in (0..t.len()).rev() {
            while st.len() > 0 && t[st[st.len() - 1]] <= t[i] {
                st.pop();
            }

            if st.len() > 0 {
                ans[i] = (st[st.len() - 1] - i) as i32;
            } else {
                ans[i] = 0;
            }

            st.push(i);
        }

        ans
    }
}
