impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut left = grid[0].len();
        let mut right = 0;
        let mut up = grid.len();
        let mut bottom = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    left = left.min(j);
                    right = right.max(j);
                    up = up.min(i);
                    bottom = bottom.max(i);
                }
            }
        }

        let width = right - left + 1;
        let height = bottom - up + 1;
        (width * height) as i32
    }
}
