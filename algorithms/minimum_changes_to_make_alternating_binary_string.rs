impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut t0 = Vec::new();
        let mut t1 = Vec::new();
        let mut x = '0';
        let mut y = '1';
        for i in 0..s.len() {
            t0.push(x);
            t1.push(y);
            let t = x;
            x = y;
            y = t;
        }
        let mut x = 0;
        let mut y = 0;
        for i in 0..s.len() {
            if s[i] != t0[i] {
                x += 1;
            }
            if s[i] != t1[i] {
                y += 1;
            }
        }
        x.min(y)
    }
}
