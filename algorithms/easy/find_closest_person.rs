impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let d1 = (x - z).abs();
        let d2 = (y - z).abs();
        if d1 == d2 {
            0
        } else if d1 < d2 {
            1
        } else {
            2
        }
    }
}
