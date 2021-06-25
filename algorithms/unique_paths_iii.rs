use std::collections::HashSet;

fn dfs(
    grid: &[Vec<i32>],
    x: i32,
    y: i32,
    passed: &mut HashSet<(i32, i32)>,
    need_to_pass: usize,
    ans: &mut i32,
) {
    let cell = grid[x as usize][y as usize];
    if cell == 2 {
        if passed.len() > need_to_pass {
            *ans += 1;
        }
        return;
    }
    if cell == -1 {
        return;
    }
    if passed.contains(&(x, y)) {
        return;
    }

    if cell == 0 || cell == 1 {
        passed.insert((x, y));
    }

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (i, j) in &dirs {
        let nx = x + i;
        let ny = y + j;
        if nx >= 0 && (nx as usize) < grid.len() && ny >= 0 && (ny as usize) < grid[0].len() {
            dfs(grid, nx, ny, passed, need_to_pass, ans);
        }
    }

    passed.remove(&(x, y));
}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut need_to_pass = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    x = i as i32;
                    y = j as i32;
                } else if grid[i][j] == 0 {
                    need_to_pass += 1;
                }
            }
        }
        let mut ans = 0;
        let mut set = HashSet::new();
        dfs(&grid, x, y, &mut set, need_to_pass, &mut ans);
        ans
    }
}
