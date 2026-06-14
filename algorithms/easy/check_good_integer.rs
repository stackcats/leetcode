impl Solution {
    pub fn check_good_integer(mut n: i32) -> bool {
        let mut digit_sum = 0;
        let mut square_sum = 0;
        while n > 0 {
            let d = n % 10;
            digit_sum += d;
            square_sum += d * d;
            n /= 10;
        }

        square_sum - digit_sum >= 50
    }
}
