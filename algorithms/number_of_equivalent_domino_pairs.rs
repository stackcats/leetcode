use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for i in 0..dominoes.len() {
            let x = dominoes[i][0].min(dominoes[i][1]);
            let y = dominoes[i][0].max(dominoes[i][1]);
            let k = format!("{},{}", x, y);
            let ct = map.entry(k).or_insert(0);
            ans += *ct;
            *ct += 1;
        }
        ans
    }
}
