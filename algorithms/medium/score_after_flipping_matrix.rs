impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len() {
            if grid[i][0] == 0 {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 1 {
                        grid[i][j] = 0;
                    } else {
                        grid[i][j] = 1;
                    }
                }
            }
        }

        for j in 1..grid[0].len() {
            let mut num_of_one = 0;
            let mut num_of_zero = 0;
            for i in 0..grid.len() {
                if grid[i][j] == 1 {
                    num_of_one += 1;
                } else {
                    num_of_zero += 1;
                }
            }
            if num_of_zero > num_of_one {
                for i in 0..grid.len() {
                    if grid[i][j] == 1 {
                        grid[i][j] = 0;
                    } else {
                        grid[i][j] = 1;
                    }
                }
            }
        }
        let mut ans = 0;
        for i in 0..grid.len() {
            let mut row_num = 0;
            let mut pow = 1;
            for j in (0..grid[i].len()).rev() {
                row_num += grid[i][j] * pow;
                pow *= 2;
            }
            ans += row_num;
        }

        ans
    }
}
