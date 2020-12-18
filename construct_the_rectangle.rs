impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let q = (area as f64).sqrt().round() as i32;
        for W in (1..=q).rev() {
            if area % W == 0 {
                return vec![area / W, W];
            }
        }
        vec![area, 1]
    }
}
