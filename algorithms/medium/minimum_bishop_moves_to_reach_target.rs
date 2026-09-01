impl Solution {
    pub fn min_bishop_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
        if (source[0] - source[1]).abs() % 2 != (target[0] - target[1]).abs() % 2 {
            return -1;
        }
        if (source[0] - target[0]).abs() == (source[1] - target[1]).abs() {
            1
        } else {
            2
        }
    }
}
