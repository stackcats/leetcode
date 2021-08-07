impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut a = 0;
        let mut b = 1;
        while n > 1 {
            let c = a + b;
            a = b;
            b = c;
            n -= 1;
        }
        b
    }
}
