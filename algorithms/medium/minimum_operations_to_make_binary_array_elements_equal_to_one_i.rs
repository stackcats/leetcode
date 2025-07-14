fn flip(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        0
    }
}

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                if i + 2 >= nums.len() {
                    return -1;
                }
                ans += 1;
                nums[i] = flip(nums[i]);
                nums[i + 1] = flip(nums[i + 1]);
                nums[i + 2] = flip(nums[i + 2]);
            }
        }
        ans
    }
}
