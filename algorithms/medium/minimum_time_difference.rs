fn time_to_minutes(t: &str) -> i32 {
    let v = t
        .split(":")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    v[0] * 60 + v[1]
}

impl Solution {
    pub fn find_min_difference(mut time_points: Vec<String>) -> i32 {
        time_points.sort_unstable();
        time_points.push(time_points[0].clone());
        let mut ans = i32::MAX;
        let min_of_day = 60 * 24;
        for i in 1..time_points.len() {
            let a = time_to_minutes(&time_points[i]);
            let b = time_to_minutes(&time_points[i - 1]);
            let diff = (a - b).abs();
            ans = ans.min(diff).min(min_of_day - diff);
        }
        ans
    }
}
