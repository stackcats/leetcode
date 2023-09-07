impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ndx = 0;
        let mut ct = 0;
        for (i, row) in mat.into_iter().enumerate() {
            let ones = row.into_iter().filter(|n| *n == 1).count();
            if ones > ct {
                ndx = i;
                ct = ones;
            }
        }
        vec![ndx as i32, ct as i32]
    }
}
