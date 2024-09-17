impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut prev = None;
        for i in (0..nums.len() - 1).step_by(2) {
            let sum = nums[i] + nums[i + 1];
            if prev.is_none() {
                prev = Some(sum);
                ans += 1;
            } else if prev.unwrap() == sum {
                ans += 1;
            } else {
                break;
            }
        }
        ans
    }
}
