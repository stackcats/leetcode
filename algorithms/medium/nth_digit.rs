impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as u32;
        let mut digit_num = 1u32;
        let mut ct = 9u32;
        while digit_num * ct < n {
            n -= digit_num * ct;
            digit_num += 1;
            ct *= 10;
        }

        let target = 10u32.pow(digit_num - 1) + (n - 1) / digit_num;
        let s = format!("{}", target);
        let c = s.as_bytes()[((n - 1) % digit_num) as usize];
        (c - b'0') as _
    }
}
