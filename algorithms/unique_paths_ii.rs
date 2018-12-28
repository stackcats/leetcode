// https://leetcode.com/problems/unique-paths-ii/description/

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: &mut Vec<Vec<i32>>) -> i32 {
        let row = obstacle_grid.len();
        let col = obstacle_grid[0].len();
        let mut d = vec![vec![0; col]; row];

        for i in 0..row {
            for j in 0..col {
                if obstacle_grid[i][j] == 1 {
                    d[i][j] = 0;
                } else {
                    if i == 0 && j == 0 {
                        d[i][j] = 1;
                    } else if i == 0 {
                        d[i][j] = d[i][j - 1];
                    } else if j == 0 {
                        d[i][j] = d[i - 1][j];
                    } else {
                        d[i][j] = d[i][j - 1] + d[i - 1][j];
                    }
                }
            }
        }

        d[row - 1][col - 1]
    }
}
