impl Solution {
    pub fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool {
        (start[0] + start[1]) % 2 == (target[0] + target[1]) % 2
    }
}
