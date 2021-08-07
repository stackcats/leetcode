use std::collections::HashMap;
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut row_map = HashMap::new();
        let mut col_map = HashMap::new();
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 {
                    let counter = row_map.entry(i).or_insert(0);
                    *counter += 1;
                    let counter = col_map.entry(j).or_insert(0);
                    *counter += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 1 && row_map[&i] == 1 && col_map[&j] == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
