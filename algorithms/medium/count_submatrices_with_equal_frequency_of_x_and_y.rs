impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let mut v = vec![vec![(0, 0); grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let (x, y) = match grid[i][j] {
                    'X' => (1, 0),
                    'Y' => (0, 1),
                    _ => (0, 0),
                };

                if j == 0 {
                    v[i][j] = (x, y);
                } else {
                    v[i][j] = (v[i][j - 1].0 + x, v[i][j - 1].1 + y);
                }
            }
        }

        let mut ans = 0;
        for j in 0..grid[0].len() {
            let (mut x, mut y) = (0, 0);
            for i in 0..grid.len() {
                x += v[i][j].0;
                y += v[i][j].1;
                if x != 0 && x == y {
                    ans += 1;
                }
            }
        }

        ans
    }
}
