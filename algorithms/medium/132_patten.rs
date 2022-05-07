impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut st: Vec<(i32, i32)> = Vec::new();
        let mut cur_min = nums[0];
        for &n in &nums[1..] {
            while !st.is_empty() && st[st.len() - 1].0 <= n {
                st.pop();
            }
            if !st.is_empty() && st[st.len() - 1].1 < n {
                return true;
            }
            st.push((n, cur_min));
            cur_min = cur_min.min(n);
        }
        
        false
    }
}
