fn to_hex(n: i32) -> char {
    let n = n as u8;
    if n < 10 {
        (b'0' + n) as char
    } else {
        (b'A' + n - 10) as char
    }
}

fn to_hex_str(mut n: i32, base: i32) -> String {
    let mut s = String::new();
    while n > 0 {
        let c = to_hex(n % base);
        s.insert(0, c);
        n /= base;
    }
    s
}

impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        format!("{}{}", to_hex_str(n * n, 16), to_hex_str(n * n * n, 36))
    }
}
