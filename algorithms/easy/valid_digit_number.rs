impl Solution {
    pub fn valid_digit(n: i32, x: i32) -> bool {
        let s = format!("{}", n);
        let x = (b'0' + (x as u8)) as char;
        let mut it = s.chars();
        it.any(|c| c == x) && it.nth(0).unwrap() != x
    }
}
