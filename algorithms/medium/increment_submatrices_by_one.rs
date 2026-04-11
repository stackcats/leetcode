impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0; n + 1]; n + 1];
        for q in queries {
            let (r1, c1, r2, c2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            diff[r1][c1] += 1;
            diff[r2 + 1][c1] -= 1;
            diff[r1][c2 + 1] -= 1;
            diff[r2 + 1][c2 + 1] += 1;
        }

        let mut mat = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                let a = if i == 0 { 0 } else { mat[i - 1][j] };
                let b = if j == 0 { 0 } else { mat[i][j - 1] };
                let c = if i == 0 || j == 0 {
                    0
                } else {
                    mat[i - 1][j - 1]
                };
                mat[i][j] = diff[i][j] + a + b - c;
            }
        }

        mat
    }
}
