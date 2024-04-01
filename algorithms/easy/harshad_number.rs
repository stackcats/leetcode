impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut y = x;
        let mut sum = 0;
        while y > 0 {
            sum += y % 10;
            y /= 10;
        }
        if x % sum == 0 { sum } else { - 1}
    }
}
