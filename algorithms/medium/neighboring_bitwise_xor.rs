impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().fold(0, |acc, n| acc ^ n) == 0
    }
}
