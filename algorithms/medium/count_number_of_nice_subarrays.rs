fn at_most(nums: &Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut l = 0;
    let mut ct = 0;
    for r in 0..nums.len() {
        ct += nums[r] % 2;
        while ct > k {
            ct -= nums[l] % 2;
            l += 1;
        }
        ans += r - l + 1;
    }

    ans as _
}

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        at_most(&nums, k) - at_most(&nums, k - 1)
    }
}
