impl Solution {
    pub fn rotate_the_box(mut box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        for i in 0..box_grid.len() {
            let mut last = box_grid[i].len();
            for j in (0..box_grid[i].len()).rev() {
                match box_grid[i][j] {
                    '*' => last = j,
                    '#' => {
                        if j < last - 1 {
                            box_grid[i][last - 1] = '#';
                            box_grid[i][j] = '.';
                            last -= 1;
                        } else {
                            last = j;
                        }
                    }
                    _ => (),
                }
            }
        }

        let mut ans = vec![vec!['.'; box_grid.len()]; box_grid[0].len()];

        for i in 0..box_grid.len() {
            for j in 0..box_grid[0].len() {
                ans[j][box_grid.len() - i - 1] = box_grid[i][j];
            }
        }
        ans
    }
}

