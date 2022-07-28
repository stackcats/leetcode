use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        for i in 0..grid.len() {
            let mut v = Vec::new();
            for j in 0..grid[i].len() {
                v.push(grid[i][j])
            }
            *map.entry(v).or_insert(0) += 1;
        }

        let mut ans = 0;
        for j in 0..grid[0].len() {
            let mut v = Vec::new();
            for i in 0..grid.len() {
                v.push(grid[i][j])
            }
            ans += map.get(&v).unwrap_or(&0);
        }
        ans
    }
}
