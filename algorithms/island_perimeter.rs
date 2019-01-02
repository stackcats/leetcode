// https://leetcode.com/problems/island-perimeter/

fn sides(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    let mut ans = 4;

    if i > 0 && grid[i - 1][j] == 1 {
        ans -= 2;
    }

    if j > 0 && grid[i][j - 1] == 1 {
        ans -= 2;
    }

    ans
}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    ans += sides(&grid, i, j);
                }
            }
        }

        ans
    }
}
