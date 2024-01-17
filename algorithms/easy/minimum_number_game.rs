impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(2) {
            nums.swap(i, i + 1);
        }
        nums
    }
}
