impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return true;
        }
        let mut ct = 0;
        for i in 0..(nums.len() - 1) {
            if nums[i] > nums[i + 1] {
                ct += 1;
            }
            if ct > 1 {
                return false;
            }
        }
        if ct == 0 {
            return true;
        }
        nums[nums.len() - 1] <= nums[0]
    }
}
