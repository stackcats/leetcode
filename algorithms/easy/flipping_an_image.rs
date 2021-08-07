impl Solution {
    pub fn flip_and_invert_image(a: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut arr = Vec::new();
        for v in a.iter() {
            let mut row = Vec::new();
            for x in v.iter().rev() {
                if *x == 0 {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }

            arr.push(row);
        }

        arr
    }
}
