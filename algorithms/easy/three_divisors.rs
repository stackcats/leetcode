impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut ct = 0;
        for i in 1..=n {
            if n % i == 0 {
                ct += 1;
                if ct > 3 {
                    return false;
                }
            }
        }
        ct == 3
    }
}
