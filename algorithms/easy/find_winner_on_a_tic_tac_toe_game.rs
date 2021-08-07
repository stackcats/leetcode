#[inline]
fn h(n: i32) -> String {
    if n == 1 {
        return "A".to_string();
    }
    "B".to_string()
}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut grid = vec![vec![0; 3]; 3];
        let mut player = 1;
        for mv in &moves {
            let i = mv[0];
            let j = mv[1];
            grid[i as usize][j as usize] = player;
            player *= -1;
        }
        if grid[0][0] == grid[0][1] && grid[0][0] == grid[0][2] && grid[0][0] != 0 {
            return h(grid[0][0]);
        }
        if grid[1][0] == grid[1][1] && grid[1][0] == grid[1][2] && grid[1][0] != 0 {
            return h(grid[1][0]);
        }
        if grid[2][0] == grid[2][1] && grid[2][0] == grid[2][2] && grid[2][0] != 0 {
            return h(grid[2][0]);
        }
        if grid[0][0] == grid[1][0] && grid[0][0] == grid[2][0] && grid[0][0] != 0 {
            return h(grid[0][0]);
        }
        if grid[0][1] == grid[1][1] && grid[0][1] == grid[2][1] && grid[0][1] != 0 {
            return h(grid[0][1]);
        }
        if grid[0][2] == grid[1][2] && grid[0][2] == grid[2][2] && grid[0][2] != 0 {
            return h(grid[0][2]);
        }
        if grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2] && grid[0][0] != 0 {
            return h(grid[0][0]);
        }
        if grid[2][0] == grid[1][1] && grid[2][0] == grid[0][2] && grid[2][0] != 0 {
            return h(grid[2][0]);
        }
        if moves.len() == 9 {
            return "Draw".to_string();
        }
        "Pending".to_string()
    }
}
