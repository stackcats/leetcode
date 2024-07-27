impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut b = 0;
                let mut w = 0;
                for (x, y) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                    if grid[i + x][j + y] == 'B' {
                        b += 1;
                    } else {
                        w += 1;
                    }
                }
                if b >= 3 || w >= 3 {
                    return true;
                }
            }
        }

        false
    }
}
