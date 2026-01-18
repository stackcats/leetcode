use std::collections::HashSet;

fn is_magic_square(grid: &Vec<Vec<i32>>, x: usize, y: usize, side: usize) -> bool {
    let mut st = HashSet::new();
    for i in 0..side {
        let mut t = 0;
        for j in 0..side {
            t += grid[i + x][j + y];
        }
        st.insert(t);
    }

    for j in 0..side {
        let mut t = 0;
        for i in 0..side {
            t += grid[i + x][j + y];
        }
        st.insert(t);
    }

    let (mut a, mut b) = (0, 0);
    for i in 0..side {
        a += grid[i + x][i + y];
        b += grid[i + x][y + side - i - 1]
    }

    st.insert(a);
    st.insert(b);

    st.len() == 1
}

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 1;
        let mut m = grid.len();
        let mut n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                let mut side = 2;
                while i + side <= m && j + side <= n {
                    if is_magic_square(&grid, i, j, side) {
                        ans = ans.max(side);
                    }
                    side += 1;
                }
            }
        }

        ans as _
    }
}
