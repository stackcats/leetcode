impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        let a = nums[0];
        let mut arr = &mut nums[1..];
        arr.sort();
        a + arr[0] + arr[1]
    }
}
