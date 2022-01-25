impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut i = 0;
        let mut j = 1;
        for &n in &nums {
            if n > 0 {
                ans[i] = n;
                i += 2;
            } else {
                ans[j] = n;
                j += 2;
            }
        }
        ans
    }
}
