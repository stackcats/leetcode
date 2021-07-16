impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = nums.clone();
        arr.extend(nums);
        arr
    }
}
