impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut ans = 0;
        let mut i = u32::pow(2, 31);
        while x > 0 {
            let r = x % 2;
            ans += r * i;
            i /= 2;
            x /= 2;
        }
        ans
    }
}
