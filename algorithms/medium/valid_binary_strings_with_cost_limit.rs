impl Solution {
    pub fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
        let mut ans = Vec::new();
        'outer: for mask in 0..(1 << n) {
            let s = format!("{:0width$b}", mask, width = n as usize);
            let t = s.as_bytes();
            let mut cost = 0;
            for i in 1..t.len() {
                if t[i] == b'1' {
                    if t[i - 1] == t[i] {
                        continue 'outer;
                    }
                    cost += i;
                }
            }
            if cost <= (k as usize) {
                ans.push(s);
            }
        }
        ans
    }
}
