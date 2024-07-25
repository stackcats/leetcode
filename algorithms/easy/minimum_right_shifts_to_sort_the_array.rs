impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut found = false;
        let mut ndx = nums.len();

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                let next = if i + 1 == nums.len() {
                    nums[i]
                } else {
                    nums[i + 1]
                };
                if next > nums[0] {
                    return -1;
                }
                if found {
                    return -1;
                }
                found = true;
                ndx = i;
            }
        }

        (nums.len() - ndx) as i32
    }
}
