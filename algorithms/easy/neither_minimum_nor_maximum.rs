impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return -1;
        }
        let mut arr = &mut nums[..3].to_vec();
        arr.sort();
        arr[1]
    }
}
