impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        dimensions
            .iter()
            .max_by_key(|d| (d[0] * d[0] + d[1] * d[1], d[0] * d[1]))
            .map(|d| d[0] * d[1])
            .unwrap()
    }
}
