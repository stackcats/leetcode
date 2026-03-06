fn bubble_up<F>(grid: &mut Vec<Vec<i32>>, mut a: usize, mut b: usize, cmp: F)
where
    F: Fn(i32, i32) -> bool,
{
    while a < grid.len() && b < grid.len() {
        let mut x = a;
        let mut y = b;
        while x > 0 && y > 0 && cmp(grid[x][y], grid[x - 1][y - 1]) {
            let t = grid[x][y];
            grid[x][y] = grid[x - 1][y - 1];
            grid[x - 1][y - 1] = t;
            x -= 1;
            y -= 1;
        }
        a += 1;
        b += 1;
    }
}

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for i in 0..grid.len() {
            bubble_up(&mut grid, i + 1, 1, |x, y| x > y);
        }

        for j in 1..grid.len() {
            bubble_up(&mut grid, 1, j + 1, |x, y| x < y);
        }

        grid
    }
}
