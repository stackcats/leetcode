impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;
        for _ in 0..32 {
            let x = a % 2;
            let y = b % 2;
            let z = c % 2;
            if z == 1 {
                if x == 0 && y == 0 {
                    ans += 1;
                }
            } else {
                ans += x + y;
            }
            a /= 2;
            b /= 2;
            c /= 2;
        }
        ans
    }
}
