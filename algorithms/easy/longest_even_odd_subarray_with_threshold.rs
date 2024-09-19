impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut res = 0;
        while l < nums.len() && r < nums.len() {
            while l < nums.len() && (nums[l] > threshold || nums[l] % 2 != 0) {
                l += 1;
            }
            if l == nums.len() {
                break;
            }
            r = l + 1;
            while r < nums.len() && nums[r] <= threshold && nums[r] % 2 != nums[r - 1] % 2 {
                r += 1;
            }
            res = res.max(r - l);
            l = r;
        }

        res as i32
    }
}
