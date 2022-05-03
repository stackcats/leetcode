impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        while l < nums.len() - 1 && nums[l] <= nums[l + 1] {
            l += 1;
        }

        let mut r = nums.len() - 1;
        while r > 0 && nums[r] >= nums[r - 1] {
            r -= 1;
        }

        if l >= r {
            return 0;
        }

        let (mi, ma) = nums[l..=r]
            .iter()
            .fold((i32::MAX, i32::MIN), |(mi, ma), &n| (mi.min(n), ma.max(n)));

        while l > 0 && mi < nums[l - 1] {
            l -= 1;
        }

        while r < nums.len() - 1 && ma > nums[r + 1] {
            r += 1;
        }

        (r - l + 1) as i32
    }
}
