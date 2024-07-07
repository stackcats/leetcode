impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; grid[0].len()];
        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                let s = format!("{}", grid[i][j]);
                ans[j] = ans[j].max(s.len() as i32);
            }
        }
        ans
    }
}
