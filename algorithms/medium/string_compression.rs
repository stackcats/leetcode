fn h(chars: &mut Vec<char>, mut ct: i32) {
    if ct <= 1 {
        return;
    }
    let mut cs = Vec::new();
    while ct > 0 {
        let n = (ct % 10) as u8;
        ct /= 10;
        cs.push((n + b'0') as char);
    }
    for c in cs.into_iter().rev() {
        chars.push(c);
    }
}

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut tmp_chars = Vec::new();
        let mut ct: i32 = 1;
        tmp_chars.push(chars[0]);
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                ct += 1;
            } else {
                h(&mut tmp_chars, ct);
                tmp_chars.push(chars[i]);
                ct = 1;
            }
        }

        h(&mut tmp_chars, ct);

        for (i, c) in tmp_chars.iter().enumerate() {
            chars[i] = *c;
        }
        tmp_chars.len() as i32
    }
}
