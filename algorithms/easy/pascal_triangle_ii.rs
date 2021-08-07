impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut up = Vec::new();

        for i in 0..row_index as usize + 1 {
            let mut curr = vec![0; i + 1];
            let mut j = 0;
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    curr[j] = 1;
                } else {
                    curr[j] = up[j - 1] + up[j];
                }
            }

            up = curr;
        }

        up
    }
}
