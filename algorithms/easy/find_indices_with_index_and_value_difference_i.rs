impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut mi = 0;
        let mut ma = 0;
        let mut index_difference = index_difference as usize;
        for i in index_difference..nums.len() {
            let j = i - index_difference;
            if nums[j] < nums[mi] {
                mi = j;
            }
            if nums[j] > nums[ma] {
                ma = j;
            }
            if nums[i] - nums[mi] >= value_difference {
                return vec![mi as i32, i as i32];
            }
            if nums[ma] - nums[i] >= value_difference {
                return vec![ma as i32, i as i32];
            }
        }
        vec![-1, -1]
    }
}
