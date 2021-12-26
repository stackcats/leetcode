impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        if num == 0 {
            return true;
        }

        if num % 10 == 0 {
            return false;
        }

        true
    }
}
