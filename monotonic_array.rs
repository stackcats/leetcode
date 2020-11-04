impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        if a.len() <= 2 {
            return true;
        }
        let mut is_increasing = true;

        // check is increasing
        for i in 1..a.len() {
            if a[i] < a[i - 1] {
                is_increasing = false;
                break;
            }
        }
        if is_increasing {
            return true;
        }

        for i in 1..a.len() {
            if a[i] > a[i - 1] {
                return false;
            }
        }
        true
    }
}
