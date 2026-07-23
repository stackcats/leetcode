impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut ct = 0;
        let mut ans = 0;
        for r in 0..nums.len() {
            if nums[r] == 0 {
                ct += 1;
            }

            while l <= r && ct > 1 {
                if nums[l] == 0 {
                    ct -= 1;
                }
                l += 1;
            }
            ans = ans.max(r - l);
        }
        ans as _
    }
}
