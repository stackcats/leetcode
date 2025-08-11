impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;
        for j in y..(y + k) {
            let mut a = x;
            let mut b = x + k - 1;
            while a < b {
                let t = grid[b][j];
                grid[b][j] = grid[a][j];
                grid[a][j] = t;
                a += 1;
                b -= 1;
            }
        }
        grid
    }
}
