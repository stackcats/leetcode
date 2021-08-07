impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..arr.len() {
            let mut j = arr.len();
            while j > 0 {
                 if (j - i) % 2 == 1 {
                for k in i..j {
                    ans += arr[k];
                }
            }
                j -= 1;
            }
        }
        ans
    }
}
