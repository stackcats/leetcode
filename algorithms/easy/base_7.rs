impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        let mut n = num.abs();
        let z = '0' as u8;
        while n > 0 {
            s.push(((n % 7) as u8 + z) as char);
            n /= 7;
        }
        if num < 0 {
            s.push('-');
        }
        s.chars().rev().collect()
    }
}
