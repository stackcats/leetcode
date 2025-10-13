impl Solution {
    pub fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ct = vec![0; 101];
        for &n in &nums {
            ct[n as usize] += 1;
        }
        ct.into_iter().enumerate().fold(0, |acc, (i, n)| {
            acc + if n % k == 0 { i as i32 * n } else { 0 }
        })
    }
}
