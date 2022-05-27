fn ct(left: usize, right: usize) -> i32 {
    if right < left + 2 {
        return 0;
    }
    let n = right - left + 1 - 2;
    ((1 + n) * n / 2) as i32
}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut ans = 0;
        let mut left = 0;
        let mut right = 1;
        for i in 2..nums.len() {
            if nums[i] - nums[i-1] == nums[i-1] - nums[i-2] {
                right = i;
            } else {
                ans += ct(left, right);
                left = i - 1;
            }
        }

        ans += ct(left, right);
        ans
    }
}
