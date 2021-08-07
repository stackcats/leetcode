impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut vec = vec![0; a.len()];
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut ndx = a.len() - 1;
        while i <= j {
            if i == j {
                vec[ndx] = a[i] * a[i];
                break;
            }
            if a[i].abs() <= a[j].abs() {
                vec[ndx] = a[j] * a[j];
                j -= 1;
            } else {
                vec[ndx] = a[i] * a[i];
                i += 1;
            }
            ndx -= 1;
        }
        vec
    }
}
