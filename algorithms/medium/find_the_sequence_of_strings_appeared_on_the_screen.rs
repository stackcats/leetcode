impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut arr = Vec::new();
        let mut s = String::new();
        for c in target.chars() {
            let mut last = 'a';
            while last < c {
                s.push(last);
                arr.push(s.to_string());
                s.pop();
                last = (last as u8 + 1) as char;
            }
            s.push(c);
            arr.push(s.to_string());
        }
        arr
    }
}
