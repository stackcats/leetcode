impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut ans = Vec::new();
        let mut n = 0;
        let mut base = 0;
        for i in 0..a.len() {
            n *= 2;
            if a[i] % 2 == 1 {
                n += 1;
            }
            n %= 10; // overflow
            if n % 5 == 0 {
                ans.push(true);
            } else {
                ans.push(false);
            }
        }
        ans
    }
}
