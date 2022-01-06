impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 1..grid.len() {
            let mut first = i32::MAX;
            let mut second = i32::MAX;

            for j in 0..grid[i].len() {
                if grid[i - 1][j] <= first {
                    second = first;
                    first = grid[i - 1][j];
                } else if grid[i - 1][j] < second {
                    second = grid[i - 1][j];
                }
            }

            for j in 0..grid[i].len() {
                if grid[i - 1][j] == first {
                    grid[i][j] += second;
                } else {
                    grid[i][j] += first;
                }
            }
        }

        *grid[grid.len() - 1].iter().min().unwrap()
    }
}
