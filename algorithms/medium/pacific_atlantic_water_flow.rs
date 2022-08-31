fn dfs(heights: &[Vec<i32>], x: usize, y: usize, map: &mut Vec<Vec<i32>>, ocean: i32) {
    if (map[x][y] & ocean) == ocean {
        return;
    }

    map[x][y] |= ocean;

    for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let nx = (x as i32 + dx) as usize;
        let ny = (y as i32 + dy) as usize;
        if nx < heights.len() && ny < heights[0].len() && heights[nx][ny] >= heights[x][y] {
            dfs(heights, nx, ny, map, map[x][y]);
        }
    }
}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();

        let mut map = vec![vec![0; n]; m];

        for i in 0..m {
            dfs(&heights, i, 0, &mut map, 1);
            dfs(&heights, i, n - 1, &mut map, 2);
        }

        for j in 0..n {
            dfs(&heights, 0, j, &mut map, 1);
            dfs(&heights, m - 1, j, &mut map, 2);
        }

        let mut ans = Vec::new();
        for i in 0..m {
            for j in 0..n {
                if map[i][j] == 3 {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }
}
