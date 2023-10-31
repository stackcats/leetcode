impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 0;
        let mut len = 0;
        while n > 0 {
            if n & 1 == 1 {
                if len % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
            n >>= 1;
            len += 1;
        }

        vec![even, odd]
    }
}
