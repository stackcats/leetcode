impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = Vec::new();
        for &n in &nums {
            if n == 2 {
                arr.push(-1);
            }
            for m in 1..=n {
                if m | (m + 1) == n {
                    arr.push(m);
                    break;
                }
            }
        }
        arr
    }
}
