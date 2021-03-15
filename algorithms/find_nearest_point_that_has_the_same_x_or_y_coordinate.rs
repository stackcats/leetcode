impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut dis = std::i32::MAX;
        let mut ans = -1;
        for i in 0..points.len() {
            let xi = points[i][0];
            let yi = points[i][1];
            if xi == x || yi == y {
                let d = (x - xi).abs() + (y - yi).abs();
                if dis > d {
                    dis = d;
                    ans = i as i32;
                }
            }
        }
        ans
    }
}
