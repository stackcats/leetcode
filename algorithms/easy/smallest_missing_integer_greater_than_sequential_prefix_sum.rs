impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut i = 1;
        while i < nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                break;
            }
            sum += nums[i];
            i += 1;
        }

        let mut arr = [0; 51];
        arr[nums[0] as usize] = 1;
        while i < nums.len() {
            arr[nums[i] as usize] = 1;
            i += 1;
        }

        while sum <= 50 {
            if arr[sum as usize] == 0 {
                break;
            }
            sum += 1;
        }
        sum
    }
}
