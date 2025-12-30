fn is_magic(g: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    let mut ct = [0; 9];
    let mut arr = [0; 3];
    for x in 0..3 {
        for y in 0..3 {
            let n = g[i + x][j + y];
            if n < 1 || n > 9 {
                return false;
            }
            ct[n as usize - 1] += 1;
            arr[x] += n;
        }
    }

    if ct.into_iter().any(|n| n != 1) {
        return false;
    }

    if !(arr[0] == arr[1] && arr[1] == arr[2]) {
        return false;
    }

    let mut arr = [0; 3];
    for x in 0..3 {
        for y in 0..3 {
            arr[x] += g[i + y][j + x];
        }
    }

    if !(arr[0] == arr[1] && arr[1] == arr[2]) {
        return false;
    }

    if g[i][j] + g[i + 2][j + 2] != g[i][j + 2] + g[i + 2][j] {
        return false;
    }

    true
}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 3 || grid[0].len() < 3 {
            return 0;
        }

        let mut ans = 0;

        for i in 0..grid.len() - 2 {
            for j in 0..grid[i].len() - 2 {
                if is_magic(&grid, i, j) {
                    ans += 1;
                }
            }
        }

        ans
    }
}
