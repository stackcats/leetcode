// https://leetcode.com/problems/search-insert-position/

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len() as i32 - 1;

        while i <= j {
            let mid = i + (j - i) / 2;
            if nums[mid as usize] == target {
                return mid;
            }

            if nums[mid as usize] < target {
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }

        i
    }
}
