impl Solution {
    pub fn spiral_matrix_iii(
        rows: i32,
        cols: i32,
        mut r_start: i32,
        mut c_start: i32,
    ) -> Vec<Vec<i32>> {
        let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut d = 0;
        let mut steps = 0;
        let mut ans = vec![vec![r_start, c_start]];
        while (ans.len() as i32) < rows * cols {
            if d == 0 || d == 2 {
                steps += 1;
            }
            for i in 0..steps {
                r_start += dir[d].0;
                c_start += dir[d].1;
                if r_start >= 0 && r_start < rows && c_start >= 0 && c_start < cols {
                    ans.push(vec![r_start, c_start]);
                }
            }

            d = (d + 1) % 4;
        }
        ans
    }
}
