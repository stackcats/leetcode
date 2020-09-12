impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut arr = Vec::new();
        let mut i = 0;
        let mut j = nums.len() / 2;
        while i < nums.len() / 2 {
            arr.push(nums[i]);
            arr.push(nums[j]);
            i += 1;
            j += 1;
        }
        arr
    }
}
