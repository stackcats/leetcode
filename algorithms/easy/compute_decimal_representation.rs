impl Solution {
    pub fn decimal_representation(mut n: i32) -> Vec<i32> {
        let mut v = Vec::new();
        let mut p = 1;
        while n > 0 {
            let d = n % 10;
            if d != 0 {
                v.push(d * p);
            }
            p *= 10;
            n /= 10;
        }
        v.into_iter().rev().collect()
    }
}
