impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut extended = vec![vec![0; grid[0].len() + 2]; grid.len() + 2];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                extended[i + 1][j + 1] = grid[i][j];
            }
        }

        let mut areas = 0;
        for i in 1..(extended.len() - 1) {
            for j in 1..(extended[i].len() - 1) {
                if extended[i][j] > 0 {
                    areas += 2;
                }
                if extended[i][j] > extended[i - 1][j] {
                    areas += extended[i][j] - extended[i - 1][j];
                }
                if extended[i][j] > extended[i + 1][j] {
                    areas += extended[i][j] - extended[i + 1][j];
                }
                if extended[i][j] > extended[i][j - 1] {
                    areas += extended[i][j] - extended[i][j - 1];
                }
                if extended[i][j] > extended[i][j + 1] {
                    areas += extended[i][j] - extended[i][j + 1];
                }
            }
        }
        areas
    }
}
