impl Solution {
    pub fn largest_integer(n: i32, mut s: i32) -> i32 {
        if n * 9 < s {
            return -1;
        }
        let mut ans = 0;
        for i in 0..n {
            let d = 9.min(s);
            ans = ans * 10 + d;
            s -= d;
        }
        ans
    }
}
