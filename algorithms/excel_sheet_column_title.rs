impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        // println!("{:?}", std::char::from_u32('A' as u32 + 1).unwrap());
        let mut buf = Vec::new();
        let start = 'A' as u8 - 1;
        while n > 0 {
            let d = (n % 26) as u8;
            if d == 0 {
                buf.insert(0, 'Z');
                n -= 1;
            } else {
                buf.insert(0, (start + d) as char);
            }
            n /= 26;
        }
        buf.iter().map(|c| (*c as char).to_string()).collect()
    }
}
