impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut diff = arr[1] - arr[0];
        for i in 2..arr.len() {
            if diff != arr[i] - arr[i-1] {
                return false;
            }
        }
        true
    }
}
