impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut team = 0;
        let mut ct = 0;
        for (i, row) in grid.into_iter().enumerate() {
            let len: i32 = row.into_iter().reduce(|acc, e| acc + e).unwrap();
            if (ct < len) {
                team = i;
                ct = len;
            }
        }
        team as i32
    }
}
