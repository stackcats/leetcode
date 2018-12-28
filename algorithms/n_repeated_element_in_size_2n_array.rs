use std::collections::HashMap;
impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut dt = HashMap::new();
        for &x in a.iter() {
            if let Some(y) = dt.get(&x) {
                return x;
            }
            dt.insert(x, 1);
        }
        -1
    }
}
