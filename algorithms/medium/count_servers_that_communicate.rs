use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_map = HashMap::new();
        let mut col_map = HashMap::new();
        let mut set = HashSet::new();
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                let s = row_map.entry(i).or_insert(HashSet::new());
                s.insert((i, j));
                let s = col_map.entry(j).or_insert(HashSet::new());
                s.insert((i, j));
            }
        }
        for v in row_map.values() {
            if v.len() >= 2 {
                for (i, j) in v.iter() {
                    set.insert((i, j));
                }
            }
        }
        for v in col_map.values() {
            if v.len() >= 2 {
                for (i, j) in v.iter() {
                    set.insert((i, j));
                }
            }
        }
        set.len() as i32
    }
}
