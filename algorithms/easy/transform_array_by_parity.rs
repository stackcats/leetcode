impl Solution {
    pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            while l < r && nums[l] % 2 == 0 {
                nums[l] = 0;
                l += 1;
            }
            while r > l && nums[r - 1] % 2 == 1 {
                nums[r - 1] = 1;
                r -= 1;
            }

            if r > 0 && l < r - 1 {
                nums[l] = 0;
                l += 1;
                nums[r - 1] = 1;
                r -= 1;
            }
        }
        nums
    }
}
