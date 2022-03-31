use std::collections::HashMap;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut arr = Vec::new();
        let mut brr = Vec::new();

        for i in 0..img1.len() {
            for j in 0..img1[i].len() {
                if img1[i][j] == 1 {
                    arr.push((i, j));
                }
                if img2[i][j] == 1 {
                    brr.push((i, j));
                }
            }
        }

        let mut map = HashMap::new();
        let mut ans = 0;
        for &(i1, j1) in &arr {
            for &(i2, j2) in &brr {
                let ct = map.entry((i1 - i2, j1 - j2)).or_insert(0);
                *ct += 1;
                ans = ans.max(*ct);
            }
        }
        ans
    }
}
