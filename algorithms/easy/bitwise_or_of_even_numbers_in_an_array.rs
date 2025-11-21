impl Solution {
    pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(0, |acc, n| if n % 2 == 0 { acc | n } else { acc })
    }
}
