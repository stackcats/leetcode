impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().fold(0, |acc, x| acc + if *x < k { 1 } else { 0 })
    }
}
