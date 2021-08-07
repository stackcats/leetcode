impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut s = number.clone();
        s.retain(|c| c != '-' && c != ' ');
        let mut t = String::new();
        let mut i = 0;
        while i + 4 < s.len() {
            for j in 0..3 {
                t.push(s.chars().nth(i + j).unwrap());
            }
            t.push('-');
            i += 3
        }
        if s.len() - i == 4 {
            t.push(s.chars().nth(i).unwrap());
            t.push(s.chars().nth(i + 1).unwrap());
            t.push('-');
            t.push(s.chars().nth(i + 2).unwrap());
            t.push(s.chars().nth(i + 3).unwrap());
        } else {
            for j in i..s.len() {
                t.push(s.chars().nth(j).unwrap());
            }
        }
        t
    }
}
