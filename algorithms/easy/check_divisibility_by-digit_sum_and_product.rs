impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut m = n;
        let mut sum = 0;
        let mut prod = 1;
        while m > 0 {
            let d = m % 10;
            sum += d;
            prod *= d;
            m /= 10;
        }
        n % (sum + prod) == 0
    }
}
