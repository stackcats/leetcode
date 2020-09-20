impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        while i < points.len() - 1 {
            let a = (points[i][0] - points[i + 1][0]).abs();
            let b = (points[i][1] - points[i + 1][1]).abs();
            ans += if a > b { a } else { b };
            i += 1;
        }
        ans
    }
}
