impl Solution {
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut st = Vec::new();
        for n in nums {
            let mut n = n as i64;
            while !st.is_empty() && st[st.len() - 1] == n {
                n *= 2;
                st.pop();
            }
            st.push(n);
        }
        st
    }
}
