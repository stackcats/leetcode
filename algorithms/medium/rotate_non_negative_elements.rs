impl Solution {
    pub fn rotate_elements(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        let arr: Vec<_> = nums.iter().copied().filter(|&v| v >= 0).collect();
        if arr.len() == 0 {
            return nums;
        }
        let mut k = (k as usize) % arr.len();
        for i in 0..nums.len() {
            if nums[i] >= 0 {
                nums[i] = arr[k];
                k = (k + 1) % arr.len();
            }
        }
        nums
    }
}
