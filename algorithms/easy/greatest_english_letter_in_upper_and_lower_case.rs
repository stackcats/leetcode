impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut lower = [false; 26];
        let mut upper = [false; 26];
        for c in s.chars() {
            if c.is_lowercase() {
                let ndx = (c as u8 - b'a') as usize;
                lower[ndx] = true;
            } else {
                let ndx = (c as u8 - b'A') as usize;
                upper[ndx] = true;
            }
        }
        
        for i in (0..26).rev() {
            if lower[i] && upper[i] {
                let c = (b'A' + (i as u8)) as char;
                return format!("{}", c);
            }
        }
        "".to_string()
    }
}
