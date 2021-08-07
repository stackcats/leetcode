fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    ((dx * dx + dy * dy) as f64).sqrt()
}

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut ans = 0_f64;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    let a = distance(points[i][0], points[i][1], points[j][0], points[j][1]);
                    let b = distance(points[i][0], points[i][1], points[k][0], points[k][1]);
                    let c = distance(points[k][0], points[k][1], points[j][0], points[j][1]);
                    let p = (a + b + c) / 2.0;
                    let s = (p * (p - a) * (p - b) * (p - c)).sqrt();
                    if ans < s {
                        ans = s;
                    }
                }
            }
        }
        ans
    }
}
