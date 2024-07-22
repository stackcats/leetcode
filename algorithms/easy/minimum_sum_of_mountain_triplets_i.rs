impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut left_min = vec![i32::MAX; nums.len()];
        let mut m: i32 = i32::MAX;
        for i in 0..nums.len() {
            m = m.min(nums[i]);
            left_min[i] = m;
        }

        let mut right_min = vec![i32::MAX; nums.len()];
        m = i32::MAX;
        for i in (0..nums.len()).rev() {
            m = m.min(nums[i]);
            right_min[i] = m;
        }

        let mut ans = i32::MAX;
        for i in 1..nums.len() - 1 {
            if left_min[i - 1] < nums[i] && nums[i] > right_min[i + 1] {
                ans = ans.min(left_min[i - 1] + nums[i] + right_min[i + 1]);
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
