impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let rc: Vec<&str> = s.split(":").collect();
        let a = rc[0].as_bytes();
        let b = rc[1].as_bytes();
        let r1 = a[0];
        let r2 = b[0];
        let c1 = a[1] - b'0';
        let c2 = b[1] - b'0';

        let mut ans = Vec::new();
        for i in r1..=r2 {
            for j in c1..=c2 {
                ans.push(format!("{}{}", i as char, j));
            }
        }
        ans
    }
}
