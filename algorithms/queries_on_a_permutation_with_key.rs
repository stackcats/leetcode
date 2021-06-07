impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut buf: Vec<i32> = (1..=m).collect();
        let mut ans = Vec::new();
        for q in &queries {
            let mut i = 0;
            while i < buf.len() {
                if buf[i] == *q {
                    break;
                }
                i += 1;
            }
            ans.push(i as i32);
            let n = buf.remove(i);
            buf.insert(0, n);
        }
        ans
    }
}
