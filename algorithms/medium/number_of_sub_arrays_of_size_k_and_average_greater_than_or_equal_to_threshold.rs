impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let uk = k as usize;
        let mut sum: i32 = arr.iter().take(uk).sum::<i32>();
        let mut ct = 0;
        if sum / k >= threshold {
            ct += 1;
        }
        for i in uk..arr.len() {
            sum -= arr[i - uk];
            sum += arr[i];
            if sum / k >= threshold {
                ct += 1;
            }
        }
        ct
    }
}
