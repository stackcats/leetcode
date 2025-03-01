impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = l + (r - l) / 2;

            if mid % 2 == 0 && nums[mid] == nums[mid + 1]
                || mid % 2 != 0 && nums[mid] == nums[mid - 1]
            {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        nums[l]
    }
}
