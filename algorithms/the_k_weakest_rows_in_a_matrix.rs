use std::collections::HashMap;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..mat.len() {
            let mut j = 0;
            while j < mat[i].len() && mat[i][j] == 1 {
                j += 1;
            }
            map.insert(i as i32, j);
        }
        let mut arr: Vec<i32> = (0..mat.len() as i32).collect();
        arr.sort_by(|a, b| {
            if map[a] == map[b] {
                a.cmp(b)
            } else {
                map[a].cmp(&map[b])
            }
        });
        arr.truncate(k as usize);
        arr
    }
}
