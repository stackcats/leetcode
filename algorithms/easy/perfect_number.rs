// https://leetcode.com/problems/perfect-number/

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut acc = 1;
        let end = (num as f64).sqrt().ceil() as i32;
        for i in 2..end {
            if num % i == 0 {
                acc += i + num / i;
            }
        }

        // 不包括1
        num != 1 && acc == num
    }
}
