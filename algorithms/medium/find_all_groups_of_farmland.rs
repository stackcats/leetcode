fn dfs(land: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize, x: &mut usize, y: &mut usize) {
    if visited[i][j] {
        return;
    }
    visited[i][j] = true;
    *x = i.max(*x);
    *y = j.max(*y);
    if i + 1 < land.len() && land[i+1][j] == 1 {
        dfs(land, visited, i + 1, j, x, y);
    }
    if j + 1 < land[0].len() && land[i][j+1] == 1 {
        dfs(land, visited, i, j + 1, x, y);
    }
}

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut visited = vec![vec![false; land[0].len()]; land.len()];
        for i in 0..land.len() {
            for j in 0..land[i].len() {
                if visited[i][j] || land[i][j] == 0 {
                    continue;
                }
                let mut x = 0;
                let mut y = 0;
                dfs(&land, &mut visited, i, j, &mut x, &mut y);
                ans.push(vec![i as i32, j as i32, x as i32, y as i32]);
            }
        }
        ans
    }
}
