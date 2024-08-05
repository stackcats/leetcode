use std::collections::HashMap;

struct neighborSum {
    grid: HashMap<i32, (i32, i32)>,
}


impl neighborSum {

    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut mp = HashMap::new();
        let adj = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let dia = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut adj_sum = 0;
               let mut dia_sum = 0; 
                for k in 0..adj.len() {
                    let (di, dj) = adj[k];
                    let ni = (i as i32 + di) as usize;
                    let nj = (j as i32 + dj) as usize;
                    if ni < grid.len() && nj < grid[0].len() {
                        adj_sum += grid[ni][nj];
                    }
                    let (di, dj) = dia[k];
                    let ni = (i as i32 + di) as usize;
                    let nj = (j as i32 + dj) as usize;
                    if ni < grid.len() && nj < grid[0].len() {
                        dia_sum += grid[ni][nj];
                    }
                }
                mp.insert(grid[i][j], (adj_sum, dia_sum));
            }
        }
        Self { grid: mp }
    }
    
    fn adjacent_sum(&self, value: i32) -> i32 {
        self.grid.get(&value).unwrap().0
    }
    
    fn diagonal_sum(&self, value: i32) -> i32 {
        self.grid.get(&value).unwrap().1
    }
}

/**
 * Your neighborSum object will be instantiated and called as such:
 * let obj = neighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

