impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        for i in 0..=(arr.len() - 3) {
            if arr[i] % 2 == 1 && arr[i+1] % 2 == 1 && arr[i+2] % 2 == 1 {
                return true;
            }
        }
        false
    }
}
