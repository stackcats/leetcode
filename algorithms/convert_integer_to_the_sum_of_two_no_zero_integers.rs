impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            if !Solution::has_zero(i) && !Solution::has_zero(n - i) {
                return vec![i, n - i];
            }
        }
        vec![]
    }
    fn has_zero(mut n: i32) -> bool {
        while n > 0 {
            if n % 10 == 0 {
                return true;
            }
            n /= 10;
        }
        false
    }
}
