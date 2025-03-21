impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut ct = 1;
        for i in 1..=n {
            let mut acc = 1;
            let mut d = 9;
            for _ in 1..i {
                acc *= d;
                d -= 1;
            }
            acc *= 9;
            ct += acc;
        }
        ct
    }
}
