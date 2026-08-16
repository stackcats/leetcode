fn distance(a: &[i32], b: &[i32]) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

impl Solution {
    pub fn nearest_drone(drones: Vec<Vec<i32>>, target: Vec<i32>) -> i32 {
        let mut dis = i32::MAX;
        let mut ndx = None;
        for (i, drone) in drones.iter().enumerate() {
            let d = distance(drone, &target);
            if d > drone[2] {
                continue;
            }
            if dis > d {
                dis = d;
                ndx = Some(i);
            }
        }
        ndx.map_or(-1, |x| x as i32)
    }
}
