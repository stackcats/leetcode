impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        for q in &queries {
            let p = q.as_bytes();
            for w in &dictionary {
                let w = w.as_bytes();
                let mut diff = 0;
                for i in 0..p.len() {
                    if p[i] != w[i] {
                        diff += 1;
                    }
                }
                if diff <= 2 {
                    ans.push(q.to_string());
                    break;
                }
            }
        }
        ans
    }
}
