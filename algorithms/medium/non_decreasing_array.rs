impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut ct = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {
                if i - 1 > 0 && i < nums.len() - 1 && nums[i+1] < nums[i-1] && nums[i-2] > nums[i] {
                    return false;
                }
                ct += 1;
                if ct > 1 {
                    return false;
                }
            }
        }

        true
    }
}
