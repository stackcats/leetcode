impl Solution {
    pub fn digit_frequency_score(mut n: i32) -> i32 {
        let mut ct = vec![0; 10];
        while n > 0 {
            ct[(n % 10) as usize] += 1;
            n /= 10;
        }
        ct.into_iter()
            .enumerate()
            .fold(0, |acc, (i, n)| acc + (i as i32) * n)
    }
}
