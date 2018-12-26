// https://leetcode.com/problems/sum-of-subarray-minimums/

impl Solution {
    pub fn sum_subarray_mins(a: Vec<i32>) -> i32 {
        // st做为单调栈当缓存使用
        let mut st = Vec::new();

        // left[i]记录从A[i-1]到A[0]有多少个连续的元素比A[i]大
        // 所以left[i]可以用来表示以A[i]为右端点的子数组中A[i]为最小点的个数
        let mut left = vec![1; a.len()];

        for i in 0..a.len() {
            while st.len() > 0 && a[st[st.len() - 1]] > a[i] {
                let last = st.pop().unwrap();
                left[i] += left[last];
            }
            st.push(i);
        }

        st.clear();

        // 同left[]
        // right[i]记录从A[i+1]到A[N]有多少个连续的元素比A[i]大
        // 所以right[i]可以用来表示以A[i]为左端点的子数组中A[i]为最小点的个数
        let mut right = vec![1; a.len()];
        for i in (0..a.len()).rev() {
            while st.len() > 0 && a[st[st.len() - 1]] >= a[i] {
                // 注意处理left时没有判断相等的情况 这里需要判断
                let last = st.pop().unwrap();
                right[i] += right[last];
            }
            st.push(i);
        }

        let mut ans = 0;
        for i in 0..a.len() {
            ans = (left[i] * right[i] * a[i] + ans) % 1000000007;
        }

        ans
    }
}
