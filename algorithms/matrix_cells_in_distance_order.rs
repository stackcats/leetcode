impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for i in 0..r {
            for j in 0..c {
                ans.push(vec![i, j]);
            }
        }
        ans.sort_by(|a, b| {
            let r1 = a[0];
            let c1 = a[1];
            let r2 = b[0];
            let c2 = b[1];
            ((r1 - r0).abs() + (c1 - c0).abs()).cmp(&((r2 - r0).abs() + (c2 - c0).abs()))
        });
        ans
    }
}
