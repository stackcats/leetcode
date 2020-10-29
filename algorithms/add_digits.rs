impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        if num % 9 == 0 {
            return 9;
        }
        num % 9
    }
}
