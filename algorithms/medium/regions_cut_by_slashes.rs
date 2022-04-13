fn build_grid(grid: &[String]) -> Vec<Vec<i32>> {
    const scale: usize = 3;
    let N = grid.len();
    let mut g = vec![vec![0; N*scale]; N*scale];
    for (i, s) in grid.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '/' {
                g[i * scale][j * scale + 2] = 1;
                g[i * scale + 1][j * scale + 1] = 1;
                g[i * scale + 2][j * scale] = 1;
            } else if c == '\\' {
                g[i * scale][j * scale] = 1;
                g[i * scale + 1][j * scale + 1] = 1;
                g[i * scale + 2][j * scale + 2] = 1;
            }
        }
    }
    g
}

fn dfs(g: &mut Vec<Vec<i32>>, i: i32, j: i32) {
    if i < 0 || i >= (g.len() as i32) || j < 0 || j >= (g.len() as i32) || g[i as usize][j as usize] == 1 {
        return;
    }

    g[i as usize][j as usize] = 1;
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for (di, dj) in dirs.into_iter() {
        dfs(g, i + di, j + dj);
    }
}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let mut g = build_grid(&grid);
        let N = g.len() as i32;
        let mut ans = 0;
        for i in 0..N {
            for j in 0..N {
                if g[i as usize][j as usize] == 0 {
                    dfs(&mut g, i, j);
                    ans += 1;
                }
                
            }
        }

        ans
    }
}
