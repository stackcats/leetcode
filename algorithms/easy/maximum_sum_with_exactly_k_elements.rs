impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.into_iter().max().unwrap();
        n * k + (k - 1) * k / 2
    }
}
