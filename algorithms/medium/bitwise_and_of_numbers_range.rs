impl Solution {
    pub fn range_bitwise_and(left: i32, mut right: i32) -> i32 {
        while right > left {
            right = right & (right - 1);
        }
        right & left
    }
}
