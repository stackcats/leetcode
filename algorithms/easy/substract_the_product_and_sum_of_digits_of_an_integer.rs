impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        let mut product = 1;
        while n > 0 {
            let d = n % 10;
            product *= d;
            sum += d;
            n /= 10;
        }
        product - sum
    }
}
